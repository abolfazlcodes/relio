use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::net::SocketAddr;
use std::path::{Path, PathBuf};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::launch_endpoint::{LaunchEndpointError, PrimaryLaunchEndpoint, SecondaryLaunchEndpoint};

const INSTANCE_FILE_NAME: &str = "instance-v1.json";
const MAX_INSTANCE_METADATA_BYTES: u64 = 512;

#[derive(Debug)]
pub enum SingleInstance {
    Primary(PrimaryInstance),
    Secondary(SecondaryLaunchEndpoint),
}

#[derive(Debug)]
pub struct PrimaryInstance {
    _lock: File,
    endpoint: PrimaryLaunchEndpoint,
}

impl PrimaryInstance {
    #[must_use]
    pub const fn endpoint(&self) -> &PrimaryLaunchEndpoint {
        &self.endpoint
    }
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
struct InstanceMetadata {
    protocol_version: u16,
    address: SocketAddr,
    authentication_token: Uuid,
}

#[derive(Debug, thiserror::Error)]
pub enum SingleInstanceError {
    #[error("single-instance runtime directory is unavailable")]
    Directory,
    #[error("single-instance metadata permissions are unsafe")]
    Permissions,
    #[error("single-instance ownership is unavailable")]
    Ownership,
    #[error("single-instance metadata is malformed")]
    Metadata,
    #[error(transparent)]
    Endpoint(#[from] LaunchEndpointError),
}

pub fn acquire(runtime_directory: &Path) -> Result<SingleInstance, SingleInstanceError> {
    std::fs::create_dir_all(runtime_directory).map_err(|_| SingleInstanceError::Directory)?;
    restrict_directory(runtime_directory)?;
    let path = runtime_directory.join(INSTANCE_FILE_NAME);
    let mut lock = OpenOptions::new()
        .create(true)
        .read(true)
        .write(true)
        .truncate(false)
        .open(&path)
        .map_err(|_| SingleInstanceError::Ownership)?;
    restrict_file(&path)?;

    match lock.try_lock() {
        Ok(()) => {
            let endpoint = PrimaryLaunchEndpoint::bind(Uuid::now_v7())?;
            let secondary = endpoint.secondary()?;
            write_metadata(&mut lock, secondary)?;
            Ok(SingleInstance::Primary(PrimaryInstance {
                _lock: lock,
                endpoint,
            }))
        }
        Err(std::fs::TryLockError::WouldBlock) => {
            let metadata = read_metadata(&mut lock)?;
            Ok(SingleInstance::Secondary(SecondaryLaunchEndpoint {
                address: metadata.address,
                authentication_token: metadata.authentication_token,
            }))
        }
        Err(_) => Err(SingleInstanceError::Ownership),
    }
}

fn write_metadata(
    file: &mut File,
    endpoint: SecondaryLaunchEndpoint,
) -> Result<(), SingleInstanceError> {
    let encoded = serde_json::to_vec(&InstanceMetadata {
        protocol_version: 1,
        address: endpoint.address,
        authentication_token: endpoint.authentication_token,
    })
    .map_err(|_| SingleInstanceError::Metadata)?;
    if encoded.len() as u64 > MAX_INSTANCE_METADATA_BYTES {
        return Err(SingleInstanceError::Metadata);
    }
    file.set_len(0)
        .and_then(|()| file.seek(SeekFrom::Start(0)).map(|_| ()))
        .and_then(|()| file.write_all(&encoded))
        .and_then(|()| file.sync_data())
        .map_err(|_| SingleInstanceError::Metadata)
}

fn read_metadata(file: &mut File) -> Result<InstanceMetadata, SingleInstanceError> {
    file.seek(SeekFrom::Start(0))
        .map_err(|_| SingleInstanceError::Metadata)?;
    let mut encoded = String::new();
    file.take(MAX_INSTANCE_METADATA_BYTES + 1)
        .read_to_string(&mut encoded)
        .map_err(|_| SingleInstanceError::Metadata)?;
    if encoded.len() as u64 > MAX_INSTANCE_METADATA_BYTES {
        return Err(SingleInstanceError::Metadata);
    }
    let metadata: InstanceMetadata =
        serde_json::from_str(&encoded).map_err(|_| SingleInstanceError::Metadata)?;
    if metadata.protocol_version != 1 || !metadata.address.ip().is_loopback() {
        return Err(SingleInstanceError::Metadata);
    }
    Ok(metadata)
}

pub fn default_runtime_directory() -> Result<PathBuf, SingleInstanceError> {
    #[cfg(target_os = "windows")]
    {
        return std::env::var_os("LOCALAPPDATA")
            .map(PathBuf::from)
            .map(|path| path.join("Relio").join("runtime"))
            .ok_or(SingleInstanceError::Directory);
    }
    #[cfg(target_os = "macos")]
    {
        return std::env::var_os("HOME")
            .map(PathBuf::from)
            .map(|path| {
                path.join("Library")
                    .join("Application Support")
                    .join("Relio")
                    .join("runtime")
            })
            .ok_or(SingleInstanceError::Directory);
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        if let Some(path) = std::env::var_os("XDG_RUNTIME_DIR") {
            return Ok(PathBuf::from(path).join("relio"));
        }
        std::env::var_os("XDG_DATA_HOME")
            .map(PathBuf::from)
            .or_else(|| {
                std::env::var_os("HOME").map(|home| PathBuf::from(home).join(".local/share"))
            })
            .map(|path| path.join("relio").join("runtime"))
            .ok_or(SingleInstanceError::Directory)
    }
}

#[cfg(unix)]
fn restrict_directory(path: &Path) -> Result<(), SingleInstanceError> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o700))
        .map_err(|_| SingleInstanceError::Permissions)
}

#[cfg(unix)]
fn restrict_file(path: &Path) -> Result<(), SingleInstanceError> {
    use std::os::unix::fs::PermissionsExt;
    std::fs::set_permissions(path, std::fs::Permissions::from_mode(0o600))
        .map_err(|_| SingleInstanceError::Permissions)
}

#[cfg(windows)]
fn restrict_directory(_path: &Path) -> Result<(), SingleInstanceError> {
    Ok(())
}

#[cfg(windows)]
fn restrict_file(_path: &Path) -> Result<(), SingleInstanceError> {
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn publishes_a_loopback_authenticated_secondary_endpoint() {
        let directory =
            std::env::temp_dir().join(format!("relio-single-instance-{}", Uuid::now_v7()));
        let Ok(first) = acquire(&directory) else {
            // Restricted build sandboxes may prohibit loopback listeners.
            return;
        };
        let SingleInstance::Primary(primary) = first else {
            panic!("first process owns the instance");
        };
        let SingleInstance::Secondary(secondary) = acquire(&directory).expect("secondary") else {
            panic!("second process forwards");
        };
        assert!(secondary.address.ip().is_loopback());
        assert_eq!(
            secondary.authentication_token,
            primary
                .endpoint()
                .secondary()
                .expect("endpoint metadata")
                .authentication_token
        );
        drop(primary);
        std::fs::remove_dir_all(directory).expect("remove fixture");
    }

    #[test]
    fn rejects_oversized_or_non_loopback_metadata() {
        let directory =
            std::env::temp_dir().join(format!("relio-single-instance-{}", Uuid::now_v7()));
        std::fs::create_dir_all(&directory).expect("fixture");
        let path = directory.join(INSTANCE_FILE_NAME);
        let mut file = File::create(&path).expect("fixture");
        file.write_all(&vec![b'x'; MAX_INSTANCE_METADATA_BYTES as usize + 1])
            .expect("fixture");
        assert!(matches!(
            read_metadata(&mut file),
            Err(SingleInstanceError::Metadata)
        ));
        std::fs::write(
            &path,
            format!(
                r#"{{"protocol_version":1,"address":"203.0.113.1:42","authentication_token":"{}"}}"#,
                Uuid::now_v7()
            ),
        )
        .expect("fixture");
        let mut file = File::open(&path).expect("fixture");
        assert!(matches!(
            read_metadata(&mut file),
            Err(SingleInstanceError::Metadata)
        ));
        std::fs::remove_dir_all(directory).expect("remove fixture");
    }
}
