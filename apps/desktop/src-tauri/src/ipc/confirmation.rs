use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use super::error::PublicError;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct ConfirmationChallenge {
    pub nonce: Uuid,
    pub operation_id: Uuid,
    pub displayed_digest: String,
    pub expires_at_unix_ms: i64,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum ConfirmationChoice {
    Approve,
    Reject,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct ConfirmationDecision {
    pub nonce: Uuid,
    pub displayed_digest: String,
    pub choice: ConfirmationChoice,
}

#[derive(Clone, Default)]
pub struct ConfirmationStore {
    active: Arc<Mutex<HashMap<Uuid, ConfirmationChallenge>>>,
}

impl ConfirmationStore {
    pub fn issue(
        &self,
        operation_id: Uuid,
        displayed_digest: String,
        expires_at_unix_ms: i64,
    ) -> ConfirmationChallenge {
        let challenge = ConfirmationChallenge {
            nonce: Uuid::now_v7(),
            operation_id,
            displayed_digest,
            expires_at_unix_ms,
        };
        self.active
            .lock()
            .expect("confirmation mutex poisoned")
            .insert(challenge.nonce, challenge.clone());
        challenge
    }

    pub fn consume(
        &self,
        decision: &ConfirmationDecision,
        now_unix_ms: i64,
    ) -> Result<ConfirmationChallenge, PublicError> {
        let challenge = self
            .active
            .lock()
            .expect("confirmation mutex poisoned")
            .remove(&decision.nonce)
            .ok_or_else(PublicError::replayed_decision)?;
        if challenge.expires_at_unix_ms < now_unix_ms
            || challenge.displayed_digest != decision.displayed_digest
        {
            return Err(PublicError::replayed_decision());
        }
        Ok(challenge)
    }
}
