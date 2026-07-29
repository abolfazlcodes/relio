use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use uuid::Uuid;

use super::error::PublicError;

const MAX_RECORDS: usize = 4_096;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct IdempotencyScope {
    pub profile_id: Option<Uuid>,
    pub command: String,
    pub target: String,
    pub key: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum IdempotencyResult {
    Reserved,
    InFlight(Uuid),
    Terminal { operation_id: Uuid, result: Vec<u8> },
}

#[derive(Clone, Debug)]
enum Entry {
    InFlight(Uuid),
    Terminal { operation_id: Uuid, result: Vec<u8> },
}

#[derive(Clone, Default)]
pub struct IdempotencyStore {
    entries: Arc<Mutex<HashMap<IdempotencyScope, Entry>>>,
}

impl IdempotencyStore {
    pub fn reserve(
        &self,
        scope: IdempotencyScope,
        operation_id: Uuid,
    ) -> Result<IdempotencyResult, PublicError> {
        let mut entries = self.entries.lock().expect("idempotency mutex poisoned");
        if let Some(existing) = entries.get(&scope) {
            return Ok(match existing {
                Entry::InFlight(id) => IdempotencyResult::InFlight(*id),
                Entry::Terminal {
                    operation_id,
                    result,
                } => IdempotencyResult::Terminal {
                    operation_id: *operation_id,
                    result: result.clone(),
                },
            });
        }
        if entries.len() >= MAX_RECORDS {
            return Err(PublicError::payload_too_large());
        }
        entries.insert(scope, Entry::InFlight(operation_id));
        Ok(IdempotencyResult::Reserved)
    }

    pub fn complete(
        &self,
        scope: &IdempotencyScope,
        operation_id: Uuid,
        result: Vec<u8>,
    ) -> Result<(), PublicError> {
        if result.len() > super::contract::MAX_METADATA_BYTES {
            return Err(PublicError::payload_too_large());
        }
        let mut entries = self.entries.lock().expect("idempotency mutex poisoned");
        match entries.get(scope) {
            Some(Entry::InFlight(id)) if *id == operation_id => {
                entries.insert(
                    scope.clone(),
                    Entry::Terminal {
                        operation_id,
                        result,
                    },
                );
                Ok(())
            }
            _ => Err(PublicError::operation_state()),
        }
    }
}
