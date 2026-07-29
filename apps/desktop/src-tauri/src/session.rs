use std::collections::BTreeMap;
use std::sync::{Arc, Mutex};

use uuid::Uuid;

pub const SESSION_WARNING_THRESHOLD: usize = 20;
pub const SESSION_HARD_LIMIT: usize = 32;
pub const SESSION_REPLAY_LIMIT_BYTES: usize = 4 * 1024 * 1024;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionLifecycleState {
    Created,
    Starting,
    Running,
    Detached,
    Failed,
    Closing,
    Closed,
}

impl SessionLifecycleState {
    #[must_use]
    pub const fn can_transition_to(self, next: Self) -> bool {
        matches!(
            (self, next),
            (Self::Created, Self::Starting)
                | (Self::Created, Self::Closing)
                | (Self::Starting, Self::Running)
                | (Self::Starting, Self::Failed)
                | (Self::Starting, Self::Closing)
                | (Self::Running, Self::Detached)
                | (Self::Running, Self::Closing)
                | (Self::Detached, Self::Running)
                | (Self::Detached, Self::Closing)
                | (Self::Failed, Self::Closing)
                | (Self::Closing, Self::Closed)
        )
    }

    #[must_use]
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Closed)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionSummary {
    pub id: Uuid,
    pub display_name: String,
    pub profile_id: String,
    pub state: SessionLifecycleState,
    pub attached: bool,
    pub process_id: Option<u32>,
    pub replay_bytes: usize,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SessionError {
    Capacity,
    Duplicate,
    InvalidTransition,
    NotFound,
    Poisoned,
}

pub trait ManagedSession: Send + Sync {
    fn summary(&self) -> Result<SessionSummary, SessionError>;
    fn close(&self) -> Result<(), SessionError>;
}

#[derive(Default)]
pub struct SessionRegistry {
    sessions: Mutex<BTreeMap<Uuid, Arc<dyn ManagedSession>>>,
}

impl SessionRegistry {
    pub fn insert(&self, session: Arc<dyn ManagedSession>) -> Result<bool, SessionError> {
        let summary = session.summary()?;
        let mut sessions = self.sessions.lock().map_err(|_| SessionError::Poisoned)?;
        if sessions.contains_key(&summary.id) {
            return Err(SessionError::Duplicate);
        }
        let live = sessions
            .values()
            .filter_map(|session| session.summary().ok())
            .filter(|summary| !summary.state.is_terminal())
            .count();
        if live >= SESSION_HARD_LIMIT {
            return Err(SessionError::Capacity);
        }
        sessions.insert(summary.id, session);
        Ok(live + 1 >= SESSION_WARNING_THRESHOLD)
    }

    pub fn get(&self, id: Uuid) -> Result<Arc<dyn ManagedSession>, SessionError> {
        self.sessions
            .lock()
            .map_err(|_| SessionError::Poisoned)?
            .get(&id)
            .cloned()
            .ok_or(SessionError::NotFound)
    }

    pub fn list(&self) -> Result<Vec<SessionSummary>, SessionError> {
        self.sessions
            .lock()
            .map_err(|_| SessionError::Poisoned)?
            .values()
            .map(|session| session.summary())
            .collect()
    }

    pub fn close(&self, id: Uuid) -> Result<(), SessionError> {
        self.get(id)?.close()
    }

    pub fn close_all(&self) -> Result<(), SessionError> {
        let sessions: Vec<_> = self
            .sessions
            .lock()
            .map_err(|_| SessionError::Poisoned)?
            .values()
            .cloned()
            .collect();
        for session in sessions {
            session.close()?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};

    static NEXT_ID: AtomicU64 = AtomicU64::new(1);

    struct FakeSession {
        id: Uuid,
        state: Mutex<SessionLifecycleState>,
    }

    impl FakeSession {
        fn new(state: SessionLifecycleState) -> Self {
            Self {
                id: Uuid::from_u128(u128::from(NEXT_ID.fetch_add(1, Ordering::Relaxed))),
                state: Mutex::new(state),
            }
        }
    }

    impl ManagedSession for FakeSession {
        fn summary(&self) -> Result<SessionSummary, SessionError> {
            Ok(SessionSummary {
                id: self.id,
                display_name: "Local shell".into(),
                profile_id: "default".into(),
                state: *self.state.lock().map_err(|_| SessionError::Poisoned)?,
                attached: true,
                process_id: None,
                replay_bytes: 0,
            })
        }

        fn close(&self) -> Result<(), SessionError> {
            let mut state = self.state.lock().map_err(|_| SessionError::Poisoned)?;
            if state.is_terminal() {
                return Ok(());
            }
            if !state.can_transition_to(SessionLifecycleState::Closing) {
                return Err(SessionError::InvalidTransition);
            }
            *state = SessionLifecycleState::Closing;
            *state = SessionLifecycleState::Closed;
            Ok(())
        }
    }

    #[test]
    fn state_table_allows_only_documented_edges() {
        use SessionLifecycleState as State;
        let states = [
            State::Created,
            State::Starting,
            State::Running,
            State::Detached,
            State::Failed,
            State::Closing,
            State::Closed,
        ];
        let allowed = [
            (State::Created, State::Starting),
            (State::Created, State::Closing),
            (State::Starting, State::Running),
            (State::Starting, State::Failed),
            (State::Starting, State::Closing),
            (State::Running, State::Detached),
            (State::Running, State::Closing),
            (State::Detached, State::Running),
            (State::Detached, State::Closing),
            (State::Failed, State::Closing),
            (State::Closing, State::Closed),
        ];
        for from in states {
            for to in states {
                assert_eq!(
                    from.can_transition_to(to),
                    allowed.contains(&(from, to)),
                    "{from:?} -> {to:?}"
                );
            }
        }
    }

    #[test]
    fn registry_warns_and_enforces_the_hard_limit() {
        let registry = SessionRegistry::default();
        for index in 1..=SESSION_HARD_LIMIT {
            let warns = registry
                .insert(Arc::new(FakeSession::new(SessionLifecycleState::Running)))
                .expect("session should fit");
            assert_eq!(warns, index >= SESSION_WARNING_THRESHOLD);
        }
        assert_eq!(
            registry.insert(Arc::new(FakeSession::new(SessionLifecycleState::Running))),
            Err(SessionError::Capacity)
        );
    }

    #[test]
    fn concurrent_close_is_idempotent_and_ends_closed() {
        let registry = Arc::new(SessionRegistry::default());
        let session = Arc::new(FakeSession::new(SessionLifecycleState::Running));
        let id = session.id;
        registry
            .insert(session)
            .expect("session should be registered");
        let threads: Vec<_> = (0..8)
            .map(|_| {
                let registry = Arc::clone(&registry);
                std::thread::spawn(move || registry.close(id))
            })
            .collect();
        for thread in threads {
            let result = thread.join().expect("close worker should not panic");
            assert!(result.is_ok() || result == Err(SessionError::InvalidTransition));
        }
        assert_eq!(
            registry
                .get(id)
                .expect("session exists")
                .summary()
                .unwrap()
                .state,
            SessionLifecycleState::Closed
        );
    }
}
