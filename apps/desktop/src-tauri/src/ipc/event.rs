use std::collections::VecDeque;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use ts_rs::TS;
use uuid::Uuid;

use super::error::PublicError;

pub const MAX_REPLAY_EVENTS: usize = 1_024;

#[derive(Clone, Debug, Deserialize, Serialize, TS)]
#[serde(deny_unknown_fields)]
#[ts(export)]
pub struct EventEnvelope<T> {
    pub contract_version: u16,
    pub subscription_id: Uuid,
    pub aggregate_type: String,
    pub aggregate_id: Uuid,
    pub sequence: u32,
    #[serde(with = "time::serde::iso8601")]
    #[ts(type = "string")]
    pub occurred_at_utc: OffsetDateTime,
    pub operation_id: Option<Uuid>,
    pub body: T,
}

#[derive(Debug)]
pub struct OrderedEventBuffer<T> {
    next_sequence: u32,
    events: VecDeque<(u32, T)>,
}

impl<T> Default for OrderedEventBuffer<T> {
    fn default() -> Self {
        Self {
            next_sequence: 1,
            events: VecDeque::with_capacity(MAX_REPLAY_EVENTS),
        }
    }
}

impl<T> OrderedEventBuffer<T> {
    pub fn push(&mut self, event: T) -> u32 {
        let sequence = self.next_sequence;
        self.next_sequence = self.next_sequence.saturating_add(1);
        if self.events.len() == MAX_REPLAY_EVENTS {
            self.events.pop_front();
        }
        self.events.push_back((sequence, event));
        sequence
    }

    pub fn replay_after(&self, sequence: u32) -> Result<Vec<(u32, &T)>, PublicError> {
        let earliest = self
            .events
            .front()
            .map_or(self.next_sequence, |item| item.0);
        if sequence.saturating_add(1) < earliest {
            return Err(PublicError::event_gap());
        }
        Ok(self
            .events
            .iter()
            .filter(|(candidate, _)| *candidate > sequence)
            .map(|(candidate, event)| (*candidate, event))
            .collect())
    }
}
