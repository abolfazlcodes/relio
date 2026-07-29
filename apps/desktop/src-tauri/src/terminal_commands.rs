#![cfg(feature = "desktop-runtime")]

use std::sync::{Arc, Mutex};
use std::time::Duration;

use tauri::Manager;
use tauri::ipc::Channel;

use crate::ipc::PublicError;
use crate::pty::{
    LocalPtySession, MAX_OUTPUT_CREDIT_BYTES, PortablePtyAdapter, PtyError, PtySize,
    discover_shell_profiles,
};
use crate::terminal_contract::{
    LocalTerminalStarted, ShellProfileSummary, StartLocalTerminalRequest, TerminalChannelEvent,
    TerminalCreditRequest, TerminalInputRequest, TerminalResizeRequest, TerminalSessionRequest,
};

#[derive(Default)]
pub struct TerminalService {
    active: Mutex<Option<Arc<LocalPtySession>>>,
}

#[tauri::command]
pub fn terminal_list_shell_profiles() -> Vec<ShellProfileSummary> {
    discover_shell_profiles()
        .into_iter()
        .map(|profile| ShellProfileSummary {
            id: profile.id,
            display_name: profile.display_name,
        })
        .collect()
}

#[tauri::command]
pub async fn terminal_start_local(
    app: tauri::AppHandle,
    request: StartLocalTerminalRequest,
    events: Channel<TerminalChannelEvent>,
) -> Result<LocalTerminalStarted, PublicError> {
    let service = app.state::<Arc<TerminalService>>().inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        let profile = discover_shell_profiles()
            .into_iter()
            .find(|profile| profile.id == request.profile_id)
            .ok_or_else(PublicError::terminal_profile)?;
        let size = PtySize {
            rows: request.rows,
            columns: request.columns,
            pixel_width: request.pixel_width,
            pixel_height: request.pixel_height,
        };
        let mut active = service
            .active
            .lock()
            .map_err(|_| PublicError::terminal_state())?;
        if active
            .as_ref()
            .is_some_and(|session| !matches!(session.state(), crate::pty::PtySessionState::Exited))
        {
            return Err(PublicError::terminal_capacity());
        }
        let session = Arc::new(
            LocalPtySession::start(&PortablePtyAdapter, &profile, size).map_err(map_pty_error)?,
        );
        let started = LocalTerminalStarted {
            session_id: session.id(),
            initial_input_sequence: "0".into(),
            maximum_output_credit_bytes: MAX_OUTPUT_CREDIT_BYTES as u32,
        };
        *active = Some(Arc::clone(&session));
        drop(active);
        stream_terminal(session, events);
        Ok(started)
    })
    .await
    .map_err(|_| PublicError::terminal_runtime())?
}

#[tauri::command]
pub fn terminal_grant_output_credit(
    app: tauri::AppHandle,
    request: TerminalCreditRequest,
) -> Result<(), PublicError> {
    with_session(&app, request.session_id, |session| {
        session
            .grant_output_credit(request.bytes as usize)
            .map_err(map_pty_error)
    })
}

#[tauri::command]
pub fn terminal_send_input(
    app: tauri::AppHandle,
    request: TerminalInputRequest,
) -> Result<(), PublicError> {
    with_session(&app, request.session_id, |session| {
        session
            .send_input(parse_sequence(&request.sequence)?, request.bytes)
            .map_err(map_pty_error)
    })
}

#[tauri::command]
pub fn terminal_resize(
    app: tauri::AppHandle,
    request: TerminalResizeRequest,
) -> Result<(), PublicError> {
    with_session(&app, request.session_id, |session| {
        session
            .resize(PtySize {
                rows: request.rows,
                columns: request.columns,
                pixel_width: request.pixel_width,
                pixel_height: request.pixel_height,
            })
            .map_err(map_pty_error)
    })
}

#[tauri::command]
pub async fn terminal_stop(
    app: tauri::AppHandle,
    request: TerminalSessionRequest,
) -> Result<(), PublicError> {
    let service = app.state::<Arc<TerminalService>>().inner().clone();
    tauri::async_runtime::spawn_blocking(move || {
        let session = find_session(&service, request.session_id)?;
        session.request_stop().map_err(map_pty_error)?;
        Ok(())
    })
    .await
    .map_err(|_| PublicError::terminal_runtime())?
}

fn with_session<T>(
    app: &tauri::AppHandle,
    session_id: uuid::Uuid,
    operation: impl FnOnce(&LocalPtySession) -> Result<T, PublicError>,
) -> Result<T, PublicError> {
    let service = app.state::<Arc<TerminalService>>();
    let session = find_session(service.inner(), session_id)?;
    operation(&session)
}

fn find_session(
    service: &TerminalService,
    session_id: uuid::Uuid,
) -> Result<Arc<LocalPtySession>, PublicError> {
    service
        .active
        .lock()
        .map_err(|_| PublicError::terminal_state())?
        .as_ref()
        .filter(|session| session.id() == session_id)
        .cloned()
        .ok_or_else(PublicError::terminal_not_found)
}

fn stream_terminal(session: Arc<LocalPtySession>, events: Channel<TerminalChannelEvent>) {
    let _ = std::thread::Builder::new()
        .name("relio-terminal-channel".into())
        .spawn(move || {
            loop {
                match session.receive_output(Duration::from_millis(100)) {
                    Ok(chunk) => {
                        if events
                            .send(TerminalChannelEvent::Output {
                                sequence: chunk.sequence.to_string(),
                                bytes: chunk.bytes,
                            })
                            .is_err()
                        {
                            let _ = session.request_stop();
                            break;
                        }
                    }
                    Err(PtyError::OutputPending) => {}
                    Err(PtyError::OutputClosed) => {
                        if let Some(result) = session.wait_for_exit(Duration::from_secs(1)) {
                            let event = match result {
                                Ok(exit) => TerminalChannelEvent::Exited {
                                    exit_code: exit.exit_code,
                                    signal: exit.signal,
                                },
                                Err(_) => TerminalChannelEvent::Failed {
                                    safe_message_key: "terminal.session_failed".into(),
                                },
                            };
                            let _ = events.send(event);
                        } else {
                            let _ = events.send(TerminalChannelEvent::Failed {
                                safe_message_key: "terminal.session_failed".into(),
                            });
                        }
                        break;
                    }
                    Err(_) => {
                        let _ = events.send(TerminalChannelEvent::Failed {
                            safe_message_key: "terminal.session_failed".into(),
                        });
                        let _ = session.request_stop();
                        break;
                    }
                }
            }
        });
}

fn parse_sequence(value: &str) -> Result<u64, PublicError> {
    if value.is_empty() || value.len() > 20 || !value.bytes().all(|byte| byte.is_ascii_digit()) {
        return Err(PublicError::terminal_request());
    }
    value.parse().map_err(|_| PublicError::terminal_request())
}

fn map_pty_error(error: PtyError) -> PublicError {
    match error {
        PtyError::InvalidDimensions
        | PtyError::InvalidShellProfile
        | PtyError::InvalidCredit
        | PtyError::InvalidInput
        | PtyError::InputSequence => PublicError::terminal_request(),
        PtyError::InputBackpressure => PublicError::terminal_backpressure(),
        PtyError::UnsupportedPlatform => PublicError::terminal_unsupported(),
        PtyError::SessionClosed | PtyError::OutputClosed | PtyError::OutputPending => {
            PublicError::terminal_state()
        }
        PtyError::OpenFailed
        | PtyError::SpawnFailed
        | PtyError::StreamFailed
        | PtyError::ResizeFailed
        | PtyError::TerminationFailed => PublicError::terminal_runtime(),
    }
}
