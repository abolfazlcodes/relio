use std::io::{Read, Write};
use std::net::{Ipv4Addr, SocketAddr, SocketAddrV4, TcpListener, TcpStream};
use std::time::Duration;

use uuid::Uuid;

use crate::lifecycle::{
    AuthenticatedLaunchIntent, LAUNCH_PROTOCOL_VERSION, LaunchIntent, LaunchIntentError,
    MAX_LAUNCH_MESSAGE_BYTES,
};

const ENDPOINT_IO_DEADLINE: Duration = Duration::from_secs(2);

#[derive(Debug)]
pub struct PrimaryLaunchEndpoint {
    listener: TcpListener,
    authentication_token: Uuid,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SecondaryLaunchEndpoint {
    pub address: SocketAddr,
    pub authentication_token: Uuid,
}

#[derive(Debug, thiserror::Error)]
pub enum LaunchEndpointError {
    #[error("local launch endpoint is unavailable")]
    Unavailable,
    #[error("local launch endpoint rejected the request")]
    Rejected,
    #[error(transparent)]
    InvalidIntent(#[from] LaunchIntentError),
}

impl PrimaryLaunchEndpoint {
    pub fn bind(authentication_token: Uuid) -> Result<Self, LaunchEndpointError> {
        let listener = TcpListener::bind(SocketAddrV4::new(Ipv4Addr::LOCALHOST, 0))
            .map_err(|_| LaunchEndpointError::Unavailable)?;
        Ok(Self {
            listener,
            authentication_token,
        })
    }

    pub fn secondary(&self) -> Result<SecondaryLaunchEndpoint, LaunchEndpointError> {
        Ok(SecondaryLaunchEndpoint {
            address: self
                .listener
                .local_addr()
                .map_err(|_| LaunchEndpointError::Unavailable)?,
            authentication_token: self.authentication_token,
        })
    }

    pub fn receive_once(&self) -> Result<AuthenticatedLaunchIntent, LaunchEndpointError> {
        let (mut stream, peer) = self
            .listener
            .accept()
            .map_err(|_| LaunchEndpointError::Unavailable)?;
        if !peer.ip().is_loopback() {
            return Err(LaunchEndpointError::Rejected);
        }
        stream
            .set_read_timeout(Some(ENDPOINT_IO_DEADLINE))
            .map_err(|_| LaunchEndpointError::Unavailable)?;
        let mut bytes = Vec::with_capacity(512);
        (&mut stream)
            .take((MAX_LAUNCH_MESSAGE_BYTES + 1) as u64)
            .read_to_end(&mut bytes)
            .map_err(|_| LaunchEndpointError::Rejected)?;
        let message =
            AuthenticatedLaunchIntent::decode_and_authenticate(&bytes, self.authentication_token)?;
        stream
            .write_all(b"accepted")
            .map_err(|_| LaunchEndpointError::Unavailable)?;
        Ok(message)
    }
}

impl SecondaryLaunchEndpoint {
    pub fn forward(
        self,
        request_id: Uuid,
        intent: LaunchIntent,
    ) -> Result<(), LaunchEndpointError> {
        if !self.address.ip().is_loopback() {
            return Err(LaunchEndpointError::Rejected);
        }
        let message = AuthenticatedLaunchIntent {
            protocol_version: LAUNCH_PROTOCOL_VERSION,
            authentication_token: self.authentication_token,
            request_id,
            intent,
        };
        let encoded = message.encode()?;
        let mut stream = TcpStream::connect_timeout(&self.address, ENDPOINT_IO_DEADLINE)
            .map_err(|_| LaunchEndpointError::Unavailable)?;
        stream
            .set_read_timeout(Some(ENDPOINT_IO_DEADLINE))
            .and_then(|()| stream.set_write_timeout(Some(ENDPOINT_IO_DEADLINE)))
            .map_err(|_| LaunchEndpointError::Unavailable)?;
        stream
            .write_all(&encoded)
            .and_then(|()| stream.shutdown(std::net::Shutdown::Write))
            .map_err(|_| LaunchEndpointError::Unavailable)?;
        let mut acknowledgement = [0_u8; 8];
        stream
            .read_exact(&mut acknowledgement)
            .map_err(|_| LaunchEndpointError::Rejected)?;
        if acknowledgement != *b"accepted" {
            return Err(LaunchEndpointError::Rejected);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use super::*;

    #[test]
    fn forwards_only_authenticated_bounded_intents_over_loopback() {
        let Ok(primary) = PrimaryLaunchEndpoint::bind(Uuid::now_v7()) else {
            return;
        };
        let secondary = primary.secondary().expect("metadata");
        let request_id = Uuid::now_v7();
        let receiver = thread::spawn(move || primary.receive_once());
        secondary
            .forward(request_id, LaunchIntent::Activate)
            .expect("forward");
        let received = receiver.join().expect("receiver").expect("accepted");
        assert_eq!(received.request_id, request_id);
        assert_eq!(received.intent, LaunchIntent::Activate);
    }

    #[test]
    fn rejects_a_spoofed_token() {
        let Ok(primary) = PrimaryLaunchEndpoint::bind(Uuid::now_v7()) else {
            return;
        };
        let mut secondary = primary.secondary().expect("metadata");
        secondary.authentication_token = Uuid::now_v7();
        let receiver = thread::spawn(move || primary.receive_once());
        assert!(
            secondary
                .forward(Uuid::now_v7(), LaunchIntent::Activate)
                .is_err()
        );
        assert!(matches!(
            receiver.join().expect("receiver"),
            Err(LaunchEndpointError::InvalidIntent(
                LaunchIntentError::AuthenticationFailed
            ))
        ));
    }
}
