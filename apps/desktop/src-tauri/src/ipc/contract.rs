use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use super::error::PublicError;

pub const CONTRACT_VERSION: u16 = 1;
pub const TARGET_METADATA_BYTES: usize = 64 * 1024;
pub const MAX_METADATA_BYTES: usize = 5 * 1024 * 1024;
pub const MAX_IDENTIFIER_BYTES: usize = 128;
pub const MAX_IDEMPOTENCY_KEY_BYTES: usize = 256;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum RequestKind {
    Query,
    Command,
    Decision,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct RequestEnvelope<T> {
    pub contract_version: u16,
    pub request_id: Uuid,
    pub window_id: String,
    pub profile_id: Option<Uuid>,
    pub expected_revision: Option<u32>,
    pub idempotency_key: Option<String>,
    pub kind: RequestKind,
    pub body: T,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(tag = "outcome", content = "body", rename_all = "snake_case")]
#[ts(export)]
pub enum ResponseOutcome<T> {
    Ok(T),
    Error(PublicError),
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct ResponseEnvelope<T> {
    pub contract_version: u16,
    pub request_id: Uuid,
    #[serde(flatten)]
    pub outcome: ResponseOutcome<T>,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct TestOperationRequest {
    pub label: String,
    pub steps: u16,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct OperationAccepted {
    pub operation_id: Uuid,
    pub replayed: bool,
}

pub fn validate_envelope<T>(request: &RequestEnvelope<T>) -> Result<(), PublicError> {
    if request.contract_version != CONTRACT_VERSION {
        return Err(PublicError::contract_unsupported());
    }
    if request.window_id.is_empty() || request.window_id.len() > MAX_IDENTIFIER_BYTES {
        return Err(PublicError::invalid_request("window_id"));
    }
    if request
        .idempotency_key
        .as_ref()
        .is_some_and(|key| key.is_empty() || key.len() > MAX_IDEMPOTENCY_KEY_BYTES)
    {
        return Err(PublicError::invalid_request("idempotency_key"));
    }
    Ok(())
}

/// Validates an optimistic-concurrency revision supplied by the caller.
///
/// # Errors
/// Returns `ipc.revision_stale` when the expected and authoritative revisions differ.
pub fn validate_revision(expected: Option<u32>, actual: u32) -> Result<(), PublicError> {
    if expected.is_some_and(|revision| revision != actual) {
        return Err(PublicError::stale_revision());
    }
    Ok(())
}

pub fn decode_request<T>(bytes: &[u8]) -> Result<RequestEnvelope<T>, PublicError>
where
    T: for<'de> Deserialize<'de>,
{
    if bytes.len() > MAX_METADATA_BYTES {
        return Err(PublicError::payload_too_large());
    }
    let request = serde_json::from_slice(bytes).map_err(|_| PublicError::malformed_request())?;
    validate_envelope(&request)?;
    Ok(request)
}
