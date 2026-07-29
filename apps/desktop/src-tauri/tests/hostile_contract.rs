use relio_desktop::ipc::{
    CONTRACT_VERSION, RequestEnvelope, RequestKind, TestOperationRequest, decode_request,
    validate_revision,
};
use uuid::Uuid;

fn valid_request() -> RequestEnvelope<TestOperationRequest> {
    RequestEnvelope {
        contract_version: CONTRACT_VERSION,
        request_id: Uuid::now_v7(),
        window_id: "main".into(),
        profile_id: None,
        expected_revision: None,
        idempotency_key: None,
        kind: RequestKind::Command,
        body: TestOperationRequest {
            label: "fixture".into(),
            steps: 1,
        },
    }
}

#[test]
fn unknown_contract_version_has_a_stable_failure() {
    let mut request = valid_request();
    request.contract_version = CONTRACT_VERSION + 1;
    let bytes = serde_json::to_vec(&request).unwrap();
    assert_eq!(
        decode_request::<TestOperationRequest>(&bytes)
            .unwrap_err()
            .code,
        "ipc.contract_unsupported"
    );
}

#[test]
fn malformed_and_unknown_envelope_fields_are_rejected() {
    assert_eq!(
        decode_request::<TestOperationRequest>(br#"{"contract_version":1}"#)
            .unwrap_err()
            .code,
        "ipc.request_malformed"
    );
    let mut value = serde_json::to_value(valid_request()).unwrap();
    value
        .as_object_mut()
        .unwrap()
        .insert("unexpected".into(), true.into());
    assert_eq!(
        decode_request::<TestOperationRequest>(&serde_json::to_vec(&value).unwrap())
            .unwrap_err()
            .code,
        "ipc.request_malformed"
    );
}

#[test]
fn stale_revision_has_a_stable_failure() {
    assert!(validate_revision(Some(7), 7).is_ok());
    assert_eq!(
        validate_revision(Some(6), 7).unwrap_err().code,
        "ipc.revision_stale"
    );
}
