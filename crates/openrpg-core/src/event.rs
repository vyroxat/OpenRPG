use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub event_type: String,
    pub payload: serde_json::Value,
}

impl Event {
    pub fn new(event_type: String, payload: serde_json::Value) -> Self {
        Self { event_type, payload }
    }
}