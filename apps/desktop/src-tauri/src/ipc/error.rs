use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum ErrorSubsystem {
    Ipc,
    Operation,
    Stream,
    Policy,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum UserAction {
    None,
    Retry,
    Refresh,
    Review,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct PublicError {
    pub code: String,
    pub subsystem: ErrorSubsystem,
    pub operation_id: Option<Uuid>,
    pub retryable: bool,
    pub user_action: UserAction,
    pub safe_message_key: String,
    pub safe_parameters: BTreeMap<String, String>,
    pub diagnostic_id: Uuid,
}

impl PublicError {
    fn new(code: &str, subsystem: ErrorSubsystem, action: UserAction) -> Self {
        Self {
            code: code.into(),
            subsystem,
            operation_id: None,
            retryable: false,
            user_action: action,
            safe_message_key: code.into(),
            safe_parameters: BTreeMap::new(),
            diagnostic_id: Uuid::now_v7(),
        }
    }

    pub fn contract_unsupported() -> Self {
        Self::new(
            "ipc.contract_unsupported",
            ErrorSubsystem::Ipc,
            UserAction::Review,
        )
    }

    pub fn malformed_request() -> Self {
        Self::new(
            "ipc.request_malformed",
            ErrorSubsystem::Ipc,
            UserAction::None,
        )
    }

    pub fn invalid_request(field: &str) -> Self {
        let mut error = Self::new(
            "ipc.request_invalid",
            ErrorSubsystem::Ipc,
            UserAction::Review,
        );
        error.safe_parameters.insert("field".into(), field.into());
        error
    }

    pub fn payload_too_large() -> Self {
        Self::new(
            "ipc.payload_too_large",
            ErrorSubsystem::Ipc,
            UserAction::None,
        )
    }

    pub fn operation_state() -> Self {
        Self::new(
            "operation.state_invalid",
            ErrorSubsystem::Operation,
            UserAction::Refresh,
        )
    }

    pub fn operation_not_found() -> Self {
        Self::new(
            "operation.not_found",
            ErrorSubsystem::Operation,
            UserAction::Refresh,
        )
    }

    pub fn stale_revision() -> Self {
        Self::new(
            "ipc.revision_stale",
            ErrorSubsystem::Ipc,
            UserAction::Refresh,
        )
    }

    pub fn event_gap() -> Self {
        Self::new("ipc.event_gap", ErrorSubsystem::Ipc, UserAction::Refresh)
    }

    pub fn stream_credit() -> Self {
        Self::new(
            "stream.credit_exceeded",
            ErrorSubsystem::Stream,
            UserAction::None,
        )
    }

    pub fn replayed_decision() -> Self {
        Self::new(
            "policy.decision_replayed",
            ErrorSubsystem::Policy,
            UserAction::Review,
        )
    }
}
