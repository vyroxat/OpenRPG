use std::collections::HashMap;
use uuid::Uuid;
use serde::{Serialize, Deserialize};

pub type EntityId = Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub id: EntityId,
    pub tags: Vec<String>,
    pub components: HashMap<String, serde_json::Value>,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

impl Entity {
    pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            tags: Vec::new(),
            components: HashMap::new(),
            metadata: None,
        }
    }
}