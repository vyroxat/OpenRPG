use std::collections::BTreeMap;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ComponentSchema {
    id: String,
}

impl ComponentSchema {
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

#[derive(Default)]
pub(crate) struct ComponentRegistry {
    schemas: BTreeMap<String, ComponentSchema>,
}

impl ComponentRegistry {
    pub(crate) fn contains(&self, id: &str) -> bool {
        self.schemas.contains_key(id)
    }
}

pub struct ComponentRegistryMut<'a> {
    pub(crate) registry: &'a mut ComponentRegistry,
}

impl ComponentRegistryMut<'_> {
    pub fn define(&mut self, schema: ComponentSchema) {
        self.registry.schemas.insert(schema.id.clone(), schema);
    }
}
