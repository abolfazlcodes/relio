#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OsSessionState {
    Active,
    Locked,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum OsSessionEvent {
    Locked,
    Unlocked,
    WebviewLost,
}

pub trait SessionAuthorityRevoker {
    fn revoke_secret_leases(&mut self);
    fn cancel_pending_confirmations(&mut self);
    fn pause_credential_operations(&mut self);
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SessionSecurityCoordinator {
    state: OsSessionState,
    epoch: u64,
}

impl Default for SessionSecurityCoordinator {
    fn default() -> Self {
        Self {
            state: OsSessionState::Active,
            epoch: 0,
        }
    }
}

impl SessionSecurityCoordinator {
    #[must_use]
    pub const fn state(&self) -> OsSessionState {
        self.state
    }

    #[must_use]
    pub const fn authority_epoch(&self) -> u64 {
        self.epoch
    }

    pub fn handle<R: SessionAuthorityRevoker>(&mut self, event: OsSessionEvent, revoker: &mut R) {
        match event {
            OsSessionEvent::Locked => {
                self.state = OsSessionState::Locked;
                self.invalidate_authority(revoker);
                revoker.revoke_secret_leases();
                revoker.pause_credential_operations();
            }
            OsSessionEvent::Unlocked => {
                // Unlock restores visibility only. It never recreates leases or
                // confirmation authority; subsequent use must reauthorize.
                self.state = OsSessionState::Active;
            }
            OsSessionEvent::WebviewLost => self.invalidate_authority(revoker),
        }
    }

    fn invalidate_authority<R: SessionAuthorityRevoker>(&mut self, revoker: &mut R) {
        self.epoch = self.epoch.saturating_add(1);
        revoker.cancel_pending_confirmations();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Default)]
    struct Revoker {
        leases: u8,
        confirmations: u8,
        operations: u8,
    }

    impl SessionAuthorityRevoker for Revoker {
        fn revoke_secret_leases(&mut self) {
            self.leases += 1;
        }

        fn cancel_pending_confirmations(&mut self) {
            self.confirmations += 1;
        }

        fn pause_credential_operations(&mut self) {
            self.operations += 1;
        }
    }

    #[test]
    fn lock_revokes_authority_and_unlock_does_not_restore_it() {
        let mut coordinator = SessionSecurityCoordinator::default();
        let mut revoker = Revoker::default();
        coordinator.handle(OsSessionEvent::Locked, &mut revoker);
        assert_eq!(coordinator.state(), OsSessionState::Locked);
        assert_eq!(
            (revoker.leases, revoker.confirmations, revoker.operations),
            (1, 1, 1)
        );
        coordinator.handle(OsSessionEvent::Unlocked, &mut revoker);
        assert_eq!(coordinator.state(), OsSessionState::Active);
        assert_eq!(
            (revoker.leases, revoker.confirmations, revoker.operations),
            (1, 1, 1)
        );
    }

    #[test]
    fn webview_loss_only_invalidates_pending_confirmation_authority() {
        let mut coordinator = SessionSecurityCoordinator::default();
        let mut revoker = Revoker::default();
        coordinator.handle(OsSessionEvent::WebviewLost, &mut revoker);
        assert_eq!(coordinator.authority_epoch(), 1);
        assert_eq!(
            (revoker.leases, revoker.confirmations, revoker.operations),
            (0, 1, 0)
        );
    }
}
