use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use super::error::PublicError;

#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum OperationState {
    Pending,
    Running,
    Cancelling,
    Completed,
    Cancelled,
    Failed,
}

impl OperationState {
    pub const fn is_terminal(self) -> bool {
        matches!(self, Self::Completed | Self::Cancelled | Self::Failed)
    }
}

#[derive(Clone, Debug)]
struct OperationRecord {
    state: OperationState,
}

#[derive(Clone, Default)]
pub struct OperationRegistry {
    records: Arc<Mutex<HashMap<Uuid, OperationRecord>>>,
}

impl OperationRegistry {
    pub fn create(&self) -> Uuid {
        let id = Uuid::now_v7();
        self.records
            .lock()
            .expect("operation registry mutex poisoned")
            .insert(
                id,
                OperationRecord {
                    state: OperationState::Pending,
                },
            );
        id
    }

    pub fn state(&self, id: Uuid) -> Result<OperationState, PublicError> {
        self.records
            .lock()
            .expect("operation registry mutex poisoned")
            .get(&id)
            .map(|record| record.state)
            .ok_or_else(PublicError::operation_not_found)
    }

    pub fn transition(
        &self,
        id: Uuid,
        next: OperationState,
    ) -> Result<OperationState, PublicError> {
        let mut records = self
            .records
            .lock()
            .expect("operation registry mutex poisoned");
        let record = records
            .get_mut(&id)
            .ok_or_else(PublicError::operation_not_found)?;
        if !valid_transition(record.state, next) {
            return Err(PublicError::operation_state());
        }
        record.state = next;
        Ok(next)
    }

    pub fn cancel(&self, id: Uuid) -> Result<OperationState, PublicError> {
        let state = self.state(id)?;
        match state {
            OperationState::Pending => self.transition(id, OperationState::Cancelled),
            OperationState::Running => self.transition(id, OperationState::Cancelling),
            OperationState::Cancelling => Ok(state),
            terminal if terminal.is_terminal() => Ok(terminal),
            _ => Err(PublicError::operation_state()),
        }
    }
}

const fn valid_transition(current: OperationState, next: OperationState) -> bool {
    matches!(
        (current, next),
        (OperationState::Pending, OperationState::Running)
            | (OperationState::Pending, OperationState::Cancelled)
            | (OperationState::Running, OperationState::Cancelling)
            | (OperationState::Running, OperationState::Completed)
            | (OperationState::Running, OperationState::Failed)
            | (OperationState::Cancelling, OperationState::Cancelled)
            | (OperationState::Cancelling, OperationState::Failed)
    )
}
