use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::engine::EngineError;
use crate::namespaced_id::NamespacedId;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentEntry {
    kind: String,
    id: String,
    data: Value,
}

impl ContentEntry {
    pub fn new(kind: impl Into<String>, id: impl Into<String>, data: Value) -> Self {
        Self {
            kind: kind.into(),
            id: id.into(),
            data,
        }
    }

    pub fn kind(&self) -> &str {
        &self.kind
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn data(&self) -> &Value {
        &self.data
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub(crate) struct ContentRegistry {
    entries: BTreeMap<String, ContentEntry>,
}

impl ContentRegistry {
    pub(crate) fn restore(entries: BTreeMap<String, ContentEntry>) -> Self {
        Self { entries }
    }

    pub(crate) fn entries(&self) -> BTreeMap<String, ContentEntry> {
        self.entries.clone()
    }

    pub(crate) fn get(&self, id: &str) -> Option<&ContentEntry> {
        self.entries.get(id)
    }

    pub(crate) fn ids_for_kind(&self, kind: &str) -> Vec<&str> {
        self.entries
            .values()
            .filter(|entry| entry.kind() == kind)
            .map(ContentEntry::id)
            .collect()
    }
}

pub struct ContentRegistryRef<'a> {
    pub(crate) registry: &'a ContentRegistry,
}

impl ContentRegistryRef<'_> {
    pub fn get(&self, id: &str) -> Option<&ContentEntry> {
        self.registry.get(id)
    }

    pub fn ids_for_kind(&self, kind: &str) -> Vec<&str> {
        self.registry.ids_for_kind(kind)
    }
}

pub struct ContentRegistryMut<'a> {
    pub(crate) registry: &'a mut ContentRegistry,
}

impl ContentRegistryMut<'_> {
    pub fn insert(&mut self, entry: ContentEntry) -> Result<(), EngineError> {
        let id = NamespacedId::parse(entry.id())?;
        let key = id.as_str().to_string();

        if self.registry.entries.contains_key(&key) {
            return Err(EngineError::validation(
                "CONTENT_ID_DUPLICATE",
                format!("content id {key} is already registered"),
            ));
        }

        self.registry.entries.insert(key, entry);
        Ok(())
    }
}
