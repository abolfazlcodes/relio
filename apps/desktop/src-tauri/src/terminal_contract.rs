use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct ShellProfileSummary {
    pub id: String,
    pub display_name: String,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct StartLocalTerminalRequest {
    pub profile_id: String,
    pub rows: u16,
    pub columns: u16,
    pub pixel_width: u16,
    pub pixel_height: u16,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct LocalTerminalStarted {
    pub session_id: Uuid,
    pub initial_input_sequence: String,
    pub maximum_output_credit_bytes: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct TerminalCreditRequest {
    pub session_id: Uuid,
    pub bytes: u32,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct TerminalInputRequest {
    pub session_id: Uuid,
    pub sequence: String,
    pub bytes: Vec<u8>,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct TerminalResizeRequest {
    pub session_id: Uuid,
    pub rows: u16,
    pub columns: u16,
    pub pixel_width: u16,
    pub pixel_height: u16,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct TerminalSessionRequest {
    pub session_id: Uuid,
}

#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize, TS)]
#[serde(rename_all = "snake_case", tag = "event")]
#[ts(export)]
pub enum TerminalChannelEvent {
    Output {
        sequence: String,
        bytes: Vec<u8>,
    },
    OutputGap {
        first_missing_sequence: String,
        next_available_sequence: String,
    },
    Exited {
        exit_code: u32,
        signal: Option<String>,
    },
    Failed {
        safe_message_key: String,
    },
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn terminal_contract_rejects_unknown_authority_shaped_fields() {
        let hostile = serde_json::json!({
            "profile_id": "default",
            "rows": 24,
            "columns": 80,
            "pixel_width": 640,
            "pixel_height": 480,
            "command": "curl attacker"
        });
        assert!(serde_json::from_value::<StartLocalTerminalRequest>(hostile).is_err());
    }
}
