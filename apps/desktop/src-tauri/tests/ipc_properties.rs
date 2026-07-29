use proptest::prelude::*;
use relio_desktop::ipc::{
    MAX_METADATA_BYTES, MAX_STREAM_CHUNK_BYTES, TestOperationRequest, TestStreamBroker,
    decode_request,
};

proptest! {
    #[test]
    fn arbitrary_metadata_never_panics_or_exposes_input(bytes in prop::collection::vec(any::<u8>(), 0..8_192)) {
        if let Err(error) = decode_request::<TestOperationRequest>(&bytes) {
            prop_assert!(!error.code.is_empty());
            prop_assert!(!error.safe_message_key.is_empty());
            prop_assert!(!error.safe_parameters.values().any(|value| !value.is_empty() && bytes.windows(value.len()).any(|candidate| candidate == value.as_bytes())));
        }
    }

    #[test]
    fn stream_never_accepts_more_than_credit(
        credit in 0_usize..MAX_STREAM_CHUNK_BYTES,
        payload_size in 0_usize..(MAX_STREAM_CHUNK_BYTES * 2),
    ) {
        let mut broker = TestStreamBroker::new(credit);
        let result = broker.send(vec![0; payload_size]);
        prop_assert_eq!(
            result.is_ok(),
            payload_size <= credit
                && payload_size <= MAX_STREAM_CHUNK_BYTES
                && payload_size <= MAX_METADATA_BYTES,
        );
    }
}
