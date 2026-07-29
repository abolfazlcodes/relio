use std::collections::VecDeque;

use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

use super::error::PublicError;

pub const MAX_STREAM_CHUNK_BYTES: usize = 64 * 1024;
pub const MAX_STREAM_QUEUE_BYTES: usize = 1024 * 1024;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum StreamGapPolicy {
    Backpressure,
    MarkGap,
    Close,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(rename_all = "snake_case")]
#[ts(export)]
pub enum StreamCloseReason {
    Completed,
    Cancelled,
    OwnerClosed,
    SourceClosed,
    OverflowGap,
    Error,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct StreamDescriptor {
    pub stream_id: Uuid,
    pub content_type: String,
    pub initial_sequence: u32,
    pub chunk_limit: u32,
    pub credit_window: u32,
    pub gap_policy: StreamGapPolicy,
}

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct StreamChunk {
    pub stream_id: Uuid,
    pub sequence: u32,
    pub flags: u8,
    #[ts(type = "Uint8Array")]
    pub bytes: Vec<u8>,
}

#[derive(Debug)]
pub struct TestStreamBroker {
    id: Uuid,
    next_sequence: u32,
    credit: usize,
    queued_bytes: usize,
    queue: VecDeque<StreamChunk>,
    closed: Option<StreamCloseReason>,
}

impl TestStreamBroker {
    pub fn new(initial_credit: usize) -> Self {
        Self {
            id: Uuid::now_v7(),
            next_sequence: 1,
            credit: initial_credit.min(MAX_STREAM_QUEUE_BYTES),
            queued_bytes: 0,
            queue: VecDeque::new(),
            closed: None,
        }
    }

    pub fn grant_credit(&mut self, bytes: usize) {
        self.credit = self
            .credit
            .saturating_add(bytes)
            .min(MAX_STREAM_QUEUE_BYTES);
    }

    pub fn send(&mut self, bytes: Vec<u8>) -> Result<u32, PublicError> {
        if self.closed.is_some()
            || bytes.len() > MAX_STREAM_CHUNK_BYTES
            || bytes.len() > self.credit
            || self.queued_bytes.saturating_add(bytes.len()) > MAX_STREAM_QUEUE_BYTES
        {
            return Err(PublicError::stream_credit());
        }
        let sequence = self.next_sequence;
        self.next_sequence = self.next_sequence.saturating_add(1);
        self.credit -= bytes.len();
        self.queued_bytes += bytes.len();
        self.queue.push_back(StreamChunk {
            stream_id: self.id,
            sequence,
            flags: 0,
            bytes,
        });
        Ok(sequence)
    }

    pub fn receive(&mut self) -> Option<StreamChunk> {
        let chunk = self.queue.pop_front()?;
        self.queued_bytes -= chunk.bytes.len();
        Some(chunk)
    }

    pub fn close(&mut self, reason: StreamCloseReason) -> bool {
        if self.closed.is_some() {
            return false;
        }
        self.closed = Some(reason);
        true
    }
}
