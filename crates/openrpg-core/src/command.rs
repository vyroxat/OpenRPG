use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub command_type: String,
    pub payload: serde_json::Value,
}

impl Command {
    pub fn new(command_type: String, payload: serde_json::Value) -> Self {
        Self { command_type, payload }
    }
}