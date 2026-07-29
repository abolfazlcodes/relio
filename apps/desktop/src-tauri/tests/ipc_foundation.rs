use relio_desktop::ipc::{
    ConfirmationChoice, ConfirmationDecision, ConfirmationStore, IdempotencyResult,
    IdempotencyScope, IdempotencyStore, MAX_METADATA_BYTES, OperationRegistry, OperationState,
    OrderedEventBuffer, StreamCloseReason, TestStreamBroker, decode_request,
};
use uuid::Uuid;

#[test]
fn hostile_metadata_is_rejected_without_echoing_input() {
    let oversized = vec![b'x'; MAX_METADATA_BYTES + 1];
    let error = decode_request::<serde_json::Value>(&oversized).expect_err("must reject");
    assert_eq!(error.code, "ipc.payload_too_large");
    assert!(!error.safe_message_key.contains('x'));
}

#[test]
fn operation_reaches_one_terminal_state_during_cancel_race() {
    let registry = OperationRegistry::default();
    let id = registry.create();
    registry.transition(id, OperationState::Running).unwrap();
    assert_eq!(registry.cancel(id).unwrap(), OperationState::Cancelling);
    assert_eq!(registry.cancel(id).unwrap(), OperationState::Cancelling);
    registry.transition(id, OperationState::Cancelled).unwrap();
    assert_eq!(registry.cancel(id).unwrap(), OperationState::Cancelled);
    assert!(registry.transition(id, OperationState::Completed).is_err());
}

#[test]
fn idempotency_reuses_terminal_result() {
    let store = IdempotencyStore::default();
    let operation_id = Uuid::now_v7();
    let scope = IdempotencyScope {
        profile_id: None,
        command: "test.start".into(),
        target: "fixture".into(),
        key: "same-intent".into(),
    };
    assert_eq!(
        store.reserve(scope.clone(), operation_id).unwrap(),
        IdempotencyResult::Reserved
    );
    store
        .complete(&scope, operation_id, b"done".to_vec())
        .unwrap();
    assert_eq!(
        store.reserve(scope, Uuid::now_v7()).unwrap(),
        IdempotencyResult::Terminal {
            operation_id,
            result: b"done".to_vec(),
        }
    );
}

#[test]
fn stale_event_cursor_requires_snapshot_recovery() {
    let mut events = OrderedEventBuffer::default();
    for value in 0..1_025 {
        events.push(value);
    }
    assert_eq!(events.replay_after(0).unwrap_err().code, "ipc.event_gap");
    assert_eq!(events.replay_after(1_024).unwrap().len(), 1);
}

#[test]
fn stream_enforces_credit_and_closes_once() {
    let mut broker = TestStreamBroker::new(3);
    assert!(broker.send(vec![1, 2, 3, 4]).is_err());
    assert_eq!(broker.send(vec![1, 2, 3]).unwrap(), 1);
    assert_eq!(broker.receive().unwrap().bytes, vec![1, 2, 3]);
    assert!(broker.close(StreamCloseReason::Completed));
    assert!(!broker.close(StreamCloseReason::Cancelled));
}

#[test]
fn confirmation_nonce_is_consumed_exactly_once() {
    let store = ConfirmationStore::default();
    let challenge = store.issue(Uuid::now_v7(), "digest".into(), 100);
    let decision = ConfirmationDecision {
        nonce: challenge.nonce,
        displayed_digest: challenge.displayed_digest.clone(),
        choice: ConfirmationChoice::Approve,
    };
    store.consume(&decision, 50).unwrap();
    assert_eq!(
        store.consume(&decision, 50).unwrap_err().code,
        "policy.decision_replayed"
    );
}
