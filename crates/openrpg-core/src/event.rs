use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct OpenRpgEvent {
    id: String,
    event_type: String,
    tick: u64,
    payload: Value,
}

impl OpenRpgEvent {
    pub fn new(
        id: impl Into<String>,
        event_type: impl Into<String>,
        tick: u64,
        payload: Value,
    ) -> Self {
        Self {
            id: id.into(),
            event_type: event_type.into(),
            tick,
            payload,
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn event_type(&self) -> &str {
        &self.event_type
    }

    pub fn tick(&self) -> u64 {
        self.tick
    }

    pub fn payload(&self) -> &Value {
        &self.payload
    }
}
