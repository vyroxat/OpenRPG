use std::collections::HashMap;
use serde_json::Value;

use crate::{Entity, EntityId, Command, Event, Patch};

#[derive(Default)]
pub struct OpenRPGCore {
    entities: HashMap<EntityId, Entity>,
    // More systems to be added per spec
}

impl OpenRPGCore {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create_entity(&mut self) -> EntityId {
        let entity = Entity::new();
        let id = entity.id;
        self.entities.insert(id, entity);
        id
    }

    pub fn execute_command(&mut self, command: Command) -> Vec<Event> {
        // Placeholder implementation
        vec![Event::new("command_executed".to_string(), serde_json::json!({"command": command}))]
    }

    pub fn tick(&mut self, delta_ms: u64) -> Vec<Patch> {
        // Placeholder
        vec![]
    }
}