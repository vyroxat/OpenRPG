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

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ContentPack {
    entries: Vec<ContentEntry>,
}

impl ContentPack {
    pub fn new(entries: Vec<ContentEntry>) -> Self {
        Self { entries }
    }

    pub fn from_json_str(input: &str) -> Result<Self, EngineError> {
        serde_json::from_str(input).map_err(|error| {
            EngineError::validation(
                "CONTENT_JSON_INVALID",
                format!("invalid content JSON: {error}"),
            )
        })
    }

    pub fn entries(&self) -> &[ContentEntry] {
        &self.entries
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

    pub(crate) fn insert(&mut self, entry: ContentEntry) -> Result<(), EngineError> {
        let id = NamespacedId::parse(entry.id())?;
        let key = id.as_str().to_string();

        if self.entries.contains_key(&key) {
            return Err(EngineError::validation(
                "CONTENT_ID_DUPLICATE",
                format!("content id {key} is already registered"),
            ));
        }

        self.entries.insert(key, entry);
        Ok(())
    }

    pub(crate) fn load_pack(&mut self, pack: ContentPack) -> Result<(), EngineError> {
        let mut next = self.clone();
        for entry in pack.entries {
            next.insert(entry)?;
        }
        *self = next;
        Ok(())
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
        self.registry.insert(entry)
    }

    pub fn load_pack(&mut self, pack: ContentPack) -> Result<(), EngineError> {
        self.registry.load_pack(pack)
    }
}
