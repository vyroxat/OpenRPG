use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::{Value, json};

use crate::component::ComponentRegistry;
use crate::engine::EngineError;
use crate::patch::StatePatch;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd, Serialize, Deserialize)]
pub struct EntityId(String);

impl EntityId {
    pub fn new(id: impl Into<String>) -> Self {
        Self(id.into())
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for EntityId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entity {
    id: EntityId,
    tags: Vec<String>,
    components: BTreeMap<String, Value>,
    metadata: Value,
}

impl Entity {
    pub fn new(id: EntityId) -> Self {
        Self {
            id,
            tags: Vec::new(),
            components: BTreeMap::new(),
            metadata: json!({}),
        }
    }

    pub fn id(&self) -> &EntityId {
        &self.id
    }

    pub fn component(&self, id: &str) -> Option<&Value> {
        self.components.get(id)
    }

    fn set_component(&mut self, id: impl Into<String>, value: Value) {
        self.components.insert(id.into(), value);
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct EntityRegistry {
    entities: BTreeMap<EntityId, Entity>,
}

impl EntityRegistry {
    pub(crate) fn get(&self, id: &EntityId) -> Option<&Entity> {
        self.entities.get(id)
    }

    pub(crate) fn restore(entities: BTreeMap<EntityId, Entity>) -> Self {
        Self { entities }
    }

    pub(crate) fn entries(&self) -> BTreeMap<EntityId, Entity> {
        self.entities.clone()
    }
}

pub struct EntityRegistryRef<'a> {
    pub(crate) registry: &'a EntityRegistry,
}

impl EntityRegistryRef<'_> {
    pub fn get(&self, id: &EntityId) -> Option<&Entity> {
        self.registry.get(id)
    }
}

pub struct EntityRegistryMut<'a> {
    pub(crate) registry: &'a mut EntityRegistry,
    pub(crate) components: &'a ComponentRegistry,
    pub(crate) patches: &'a mut Vec<StatePatch>,
}

impl EntityRegistryMut<'_> {
    pub fn create_with_component(
        &mut self,
        id: EntityId,
        component_id: &str,
        component: Value,
    ) -> Result<EntityId, EngineError> {
        if !self.components.contains(component_id) {
            return Err(EngineError::validation(
                "UNKNOWN_COMPONENT",
                format!("component {component_id} has not been registered"),
            ));
        }

        let mut entity = Entity::new(id.clone());
        entity.set_component(component_id, component);
        let value = serde_json::to_value(&entity)
            .map_err(|error| EngineError::internal("ENTITY_SERIALIZE_FAILED", error.to_string()))?;

        self.registry.entities.insert(id.clone(), entity);
        self.patches
            .push(StatePatch::add(format!("/entities/{id}"), value));

        Ok(id)
    }
}
