use std::time::Duration;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub const MAX_LAUNCH_MESSAGE_BYTES: usize = 4 * 1024;
pub const LAUNCH_PROTOCOL_VERSION: u16 = 1;
pub const GRACEFUL_SHUTDOWN_DEADLINE: Duration = Duration::from_secs(10);
pub const CHILD_STOP_DEADLINE: Duration = Duration::from_secs(3);
pub const MIN_WINDOW_WIDTH: u32 = 720;
pub const MIN_WINDOW_HEIGHT: u32 = 480;
pub const MAX_WINDOW_DIMENSION: u32 = 16_384;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case", tag = "kind", deny_unknown_fields)]
pub enum LaunchIntent {
    Activate,
    OpenWorkspace { workspace_id: Uuid },
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AuthenticatedLaunchIntent {
    pub protocol_version: u16,
    pub authentication_token: Uuid,
    pub request_id: Uuid,
    pub intent: LaunchIntent,
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum LaunchIntentError {
    #[error("launch message exceeds the protocol limit")]
    TooLarge,
    #[error("launch message is malformed")]
    Malformed,
    #[error("launch protocol version is unsupported")]
    UnsupportedVersion,
    #[error("launch endpoint authentication failed")]
    AuthenticationFailed,
}

impl AuthenticatedLaunchIntent {
    pub fn encode(&self) -> Result<Vec<u8>, LaunchIntentError> {
        let encoded = serde_json::to_vec(self).map_err(|_| LaunchIntentError::Malformed)?;
        if encoded.len() > MAX_LAUNCH_MESSAGE_BYTES {
            return Err(LaunchIntentError::TooLarge);
        }
        Ok(encoded)
    }

    pub fn decode_and_authenticate(
        bytes: &[u8],
        expected_token: Uuid,
    ) -> Result<Self, LaunchIntentError> {
        if bytes.len() > MAX_LAUNCH_MESSAGE_BYTES {
            return Err(LaunchIntentError::TooLarge);
        }
        validate_launch_shape(bytes)?;
        let message: Self =
            serde_json::from_slice(bytes).map_err(|_| LaunchIntentError::Malformed)?;
        if message.protocol_version != LAUNCH_PROTOCOL_VERSION {
            return Err(LaunchIntentError::UnsupportedVersion);
        }
        if message.authentication_token != expected_token {
            return Err(LaunchIntentError::AuthenticationFailed);
        }
        Ok(message)
    }
}

fn validate_launch_shape(bytes: &[u8]) -> Result<(), LaunchIntentError> {
    let value: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|_| LaunchIntentError::Malformed)?;
    let object = value.as_object().ok_or(LaunchIntentError::Malformed)?;
    let allowed = [
        "protocol_version",
        "authentication_token",
        "request_id",
        "intent",
    ];
    if object.len() != allowed.len() || !object.keys().all(|key| allowed.contains(&key.as_str())) {
        return Err(LaunchIntentError::Malformed);
    }
    let intent = object
        .get("intent")
        .and_then(serde_json::Value::as_object)
        .ok_or(LaunchIntentError::Malformed)?;
    let kind = intent
        .get("kind")
        .and_then(serde_json::Value::as_str)
        .ok_or(LaunchIntentError::Malformed)?;
    let valid = match kind {
        "activate" => intent.len() == 1,
        "open_workspace" => intent.len() == 2 && intent.contains_key("workspace_id"),
        _ => false,
    };
    valid.then_some(()).ok_or(LaunchIntentError::Malformed)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StartupState {
    Bootstrapping,
    SingleInstance,
    ForwardAndExit,
    PlatformReady,
    ProfileLocked,
    OpeningProfile,
    Migrating,
    Recovering,
    RestoringWorkbench,
    Ready,
    RecoveryMode,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StartupEvent {
    LoggerReady,
    PrimaryInstance,
    ExistingPrimary,
    PlatformReady,
    UnlockApproved,
    SchemaCurrent,
    SchemaBehind,
    MigrationComplete,
    RecoveryComplete,
    WorkbenchRestored,
    ProfileFailure,
    MigrationFailure,
    IntegrityFailure,
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("startup event {event:?} is invalid while in {state:?}")]
pub struct StartupTransitionError {
    pub state: StartupState,
    pub event: StartupEvent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StartupCoordinator {
    state: StartupState,
}

impl Default for StartupCoordinator {
    fn default() -> Self {
        Self {
            state: StartupState::Bootstrapping,
        }
    }
}

impl StartupCoordinator {
    #[must_use]
    pub const fn state(&self) -> StartupState {
        self.state
    }

    pub fn transition(
        &mut self,
        event: StartupEvent,
    ) -> Result<StartupState, StartupTransitionError> {
        use StartupEvent as E;
        use StartupState as S;
        let next = match (self.state, event) {
            (S::Bootstrapping, E::LoggerReady) => S::SingleInstance,
            (S::SingleInstance, E::ExistingPrimary) => S::ForwardAndExit,
            (S::SingleInstance, E::PrimaryInstance) => S::PlatformReady,
            (S::PlatformReady, E::PlatformReady) => S::ProfileLocked,
            (S::ProfileLocked, E::UnlockApproved) => S::OpeningProfile,
            (S::OpeningProfile, E::SchemaBehind) => S::Migrating,
            (S::OpeningProfile, E::SchemaCurrent) | (S::Migrating, E::MigrationComplete) => {
                S::Recovering
            }
            (S::Recovering, E::RecoveryComplete) => S::RestoringWorkbench,
            (S::RestoringWorkbench, E::WorkbenchRestored) => S::Ready,
            (S::OpeningProfile, E::ProfileFailure)
            | (S::Migrating, E::MigrationFailure)
            | (S::Recovering, E::IntegrityFailure) => S::RecoveryMode,
            (state, event) => return Err(StartupTransitionError { state, event }),
        };
        self.state = next;
        Ok(next)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PreviousExit {
    Clean,
    Unclean,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RecoveryMarker {
    clean_exit: bool,
}

impl RecoveryMarker {
    #[must_use]
    pub const fn created_for_startup() -> Self {
        Self { clean_exit: false }
    }

    pub const fn mark_clean(&mut self) {
        self.clean_exit = true;
    }

    #[must_use]
    pub const fn previous_exit(&self) -> PreviousExit {
        if self.clean_exit {
            PreviousExit::Clean
        } else {
            PreviousExit::Unclean
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct DisplayBounds {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WindowGeometry {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub maximized: bool,
}

impl WindowGeometry {
    #[must_use]
    pub fn restore(self, displays: &[DisplayBounds], fallback: Self) -> Self {
        if self.width < MIN_WINDOW_WIDTH
            || self.height < MIN_WINDOW_HEIGHT
            || self.width > MAX_WINDOW_DIMENSION
            || self.height > MAX_WINDOW_DIMENSION
            || !displays.iter().any(|display| intersects(self, *display))
        {
            return fallback;
        }
        self
    }
}

fn intersects(window: WindowGeometry, display: DisplayBounds) -> bool {
    let right = i64::from(window.x) + i64::from(window.width);
    let bottom = i64::from(window.y) + i64::from(window.height);
    let display_right = i64::from(display.x) + i64::from(display.width);
    let display_bottom = i64::from(display.y) + i64::from(display.height);
    i64::from(window.x) < display_right
        && right > i64::from(display.x)
        && i64::from(window.y) < display_bottom
        && bottom > i64::from(display.y)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShutdownState {
    Running,
    Reviewing,
    Quiescing,
    Draining,
    ForcedCleanup,
    Persisting,
    Exiting,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ShutdownEvent {
    CloseRequested,
    Cancelled,
    Confirmed,
    Quiesced,
    Drained,
    DeadlineElapsed,
    CleanupComplete,
    Persisted,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CloseBlockers {
    pub dirty_remote_buffers: u16,
    pub transfers: u16,
    pub tunnels: u16,
    pub recordings: u16,
    pub protected_migration: bool,
}

impl CloseBlockers {
    #[must_use]
    pub const fn requires_review(self) -> bool {
        self.dirty_remote_buffers > 0
            || self.transfers > 0
            || self.tunnels > 0
            || self.recordings > 0
            || self.protected_migration
    }
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
#[error("shutdown event {event:?} is invalid while in {state:?}")]
pub struct ShutdownTransitionError {
    pub state: ShutdownState,
    pub event: ShutdownEvent,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ShutdownCoordinator {
    state: ShutdownState,
}

impl Default for ShutdownCoordinator {
    fn default() -> Self {
        Self {
            state: ShutdownState::Running,
        }
    }
}

impl ShutdownCoordinator {
    #[must_use]
    pub const fn state(&self) -> ShutdownState {
        self.state
    }

    pub fn transition(
        &mut self,
        event: ShutdownEvent,
    ) -> Result<ShutdownState, ShutdownTransitionError> {
        use ShutdownEvent as E;
        use ShutdownState as S;
        let next = match (self.state, event) {
            (S::Running, E::CloseRequested) => S::Reviewing,
            (S::Reviewing, E::Cancelled) => S::Running,
            (S::Reviewing, E::Confirmed) => S::Quiescing,
            (S::Quiescing, E::Quiesced) => S::Draining,
            (S::Draining, E::Drained) => S::Persisting,
            (S::Draining, E::DeadlineElapsed) => S::ForcedCleanup,
            (S::ForcedCleanup, E::CleanupComplete) => S::Persisting,
            (S::Persisting, E::Persisted) => S::Exiting,
            (state, event) => return Err(ShutdownTransitionError { state, event }),
        };
        self.state = next;
        Ok(next)
    }

    pub fn begin_unblocked_shutdown(&mut self) -> Result<(), ShutdownTransitionError> {
        self.transition(ShutdownEvent::CloseRequested)?;
        self.transition(ShutdownEvent::Confirmed)?;
        self.transition(ShutdownEvent::Quiesced)?;
        self.transition(ShutdownEvent::Drained)?;
        self.transition(ShutdownEvent::Persisted)?;
        Ok(())
    }

    pub fn on_session_lock(&mut self) {
        self.invalidate_pending_confirmation();
    }

    pub fn on_webview_lost(&mut self) {
        self.invalidate_pending_confirmation();
    }

    fn invalidate_pending_confirmation(&mut self) {
        if self.state == ShutdownState::Reviewing {
            self.state = ShutdownState::Running;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn message(intent: LaunchIntent) -> AuthenticatedLaunchIntent {
        AuthenticatedLaunchIntent {
            protocol_version: LAUNCH_PROTOCOL_VERSION,
            authentication_token: Uuid::now_v7(),
            request_id: Uuid::now_v7(),
            intent,
        }
    }

    #[test]
    fn authenticates_only_bounded_allowed_launch_intents() {
        let message = message(LaunchIntent::Activate);
        let encoded = message.encode().expect("bounded intent");
        assert_eq!(
            AuthenticatedLaunchIntent::decode_and_authenticate(
                &encoded,
                message.authentication_token
            ),
            Ok(message.clone())
        );
        assert_eq!(
            AuthenticatedLaunchIntent::decode_and_authenticate(&encoded, Uuid::now_v7()),
            Err(LaunchIntentError::AuthenticationFailed)
        );
        assert_eq!(
            AuthenticatedLaunchIntent::decode_and_authenticate(
                &vec![b'x'; MAX_LAUNCH_MESSAGE_BYTES + 1],
                message.authentication_token
            ),
            Err(LaunchIntentError::TooLarge)
        );
    }

    #[test]
    fn rejects_unknown_launch_fields_and_versions() {
        let token = Uuid::now_v7();
        let hostile = format!(
            r#"{{"protocol_version":1,"authentication_token":"{token}","request_id":"{}","intent":{{"kind":"activate","path":"/etc/shadow"}}}}"#,
            Uuid::now_v7()
        );
        assert_eq!(
            AuthenticatedLaunchIntent::decode_and_authenticate(hostile.as_bytes(), token),
            Err(LaunchIntentError::Malformed)
        );
        let mut unsupported = message(LaunchIntent::Activate);
        unsupported.protocol_version += 1;
        let encoded = unsupported.encode().expect("bounded");
        assert_eq!(
            AuthenticatedLaunchIntent::decode_and_authenticate(
                &encoded,
                unsupported.authentication_token
            ),
            Err(LaunchIntentError::UnsupportedVersion)
        );
    }

    #[test]
    fn follows_startup_order_and_rejects_invalid_transitions() {
        let mut startup = StartupCoordinator::default();
        assert!(startup.transition(StartupEvent::UnlockApproved).is_err());
        for event in [
            StartupEvent::LoggerReady,
            StartupEvent::PrimaryInstance,
            StartupEvent::PlatformReady,
            StartupEvent::UnlockApproved,
            StartupEvent::SchemaCurrent,
            StartupEvent::RecoveryComplete,
            StartupEvent::WorkbenchRestored,
        ] {
            startup.transition(event).expect("valid transition");
        }
        assert_eq!(startup.state(), StartupState::Ready);
    }

    #[test]
    fn forced_termination_is_not_reported_as_clean() {
        let mut marker = RecoveryMarker::created_for_startup();
        assert_eq!(marker.previous_exit(), PreviousExit::Unclean);
        marker.mark_clean();
        assert_eq!(marker.previous_exit(), PreviousExit::Clean);
    }

    #[test]
    fn restores_only_safe_visible_window_geometry() {
        let fallback = WindowGeometry {
            x: 80,
            y: 80,
            width: 1120,
            height: 720,
            maximized: false,
        };
        let display = DisplayBounds {
            x: 0,
            y: 0,
            width: 1920,
            height: 1080,
        };
        assert_eq!(fallback.restore(&[display], fallback), fallback);
        assert_eq!(
            WindowGeometry {
                x: 9000,
                y: 9000,
                ..fallback
            }
            .restore(&[display], fallback),
            fallback
        );
    }

    #[test]
    fn shutdown_cancels_review_or_escalates_at_deadline() {
        let mut shutdown = ShutdownCoordinator::default();
        shutdown
            .transition(ShutdownEvent::CloseRequested)
            .expect("review");
        shutdown.on_session_lock();
        assert_eq!(shutdown.state(), ShutdownState::Running);
        for event in [
            ShutdownEvent::CloseRequested,
            ShutdownEvent::Confirmed,
            ShutdownEvent::Quiesced,
            ShutdownEvent::DeadlineElapsed,
            ShutdownEvent::CleanupComplete,
            ShutdownEvent::Persisted,
        ] {
            shutdown.transition(event).expect("valid transition");
        }
        assert_eq!(shutdown.state(), ShutdownState::Exiting);
        assert_eq!(GRACEFUL_SHUTDOWN_DEADLINE, Duration::from_secs(10));
        assert_eq!(CHILD_STOP_DEADLINE, Duration::from_secs(3));
    }

    #[test]
    fn close_blockers_are_explicit() {
        assert!(
            CloseBlockers {
                transfers: 1,
                ..CloseBlockers::default()
            }
            .requires_review()
        );
        assert!(!CloseBlockers::default().requires_review());
    }
}
