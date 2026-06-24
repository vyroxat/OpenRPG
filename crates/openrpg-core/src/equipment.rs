use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

use crate::engine::EngineError;
use crate::namespaced_id::NamespacedId;
use crate::stats::StatBlock;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EquipmentStatModifier {
    stat_id: String,
    value: f64,
}

impl EquipmentStatModifier {
    pub fn new(stat_id: impl Into<String>, value: f64) -> Self {
        Self {
            stat_id: stat_id.into(),
            value,
        }
    }

    pub fn stat_id(&self) -> &str {
        &self.stat_id
    }

    pub fn value(&self) -> f64 {
        self.value
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct EquipmentDefinition {
    item_id: String,
    allowed_slots: Vec<String>,
    flat_modifiers: Vec<EquipmentStatModifier>,
}

impl EquipmentDefinition {
    pub fn new<const N: usize>(item_id: impl Into<String>, allowed_slots: [&str; N]) -> Self {
        Self {
            item_id: item_id.into(),
            allowed_slots: allowed_slots.into_iter().map(ToString::to_string).collect(),
            flat_modifiers: Vec::new(),
        }
    }

    pub fn with_flat_modifier(mut self, stat_id: impl Into<String>, value: f64) -> Self {
        self.flat_modifiers
            .push(EquipmentStatModifier::new(stat_id, value));
        self
    }

    pub fn item_id(&self) -> &str {
        &self.item_id
    }

    pub fn allows_slot(&self, slot_id: &str) -> bool {
        self.allowed_slots.iter().any(|slot| slot == slot_id)
    }

    pub fn flat_modifiers(&self) -> &[EquipmentStatModifier] {
        &self.flat_modifiers
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Equipment {
    slots: BTreeSet<String>,
    equipped: BTreeMap<String, String>,
}

impl Equipment {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define_slot(&mut self, slot_id: impl Into<String>) {
        self.slots.insert(slot_id.into());
    }

    pub fn equip(
        &mut self,
        definition: &EquipmentDefinition,
        slot_id: &str,
    ) -> Result<Option<String>, EngineError> {
        NamespacedId::parse(definition.item_id())?;
        if !self.slots.contains(slot_id) {
            return Err(EngineError::validation(
                "EQUIPMENT_SLOT_UNKNOWN",
                format!("equipment slot {slot_id} is not defined"),
            ));
        }
        if !definition.allows_slot(slot_id) {
            return Err(EngineError::validation(
                "EQUIPMENT_SLOT_INVALID",
                format!(
                    "item {} cannot equip in slot {slot_id}",
                    definition.item_id()
                ),
            ));
        }

        Ok(self
            .equipped
            .insert(slot_id.to_string(), definition.item_id().to_string()))
    }

    pub fn unequip(&mut self, slot_id: &str) -> Result<Option<String>, EngineError> {
        if !self.slots.contains(slot_id) {
            return Err(EngineError::validation(
                "EQUIPMENT_SLOT_UNKNOWN",
                format!("equipment slot {slot_id} is not defined"),
            ));
        }

        Ok(self.equipped.remove(slot_id))
    }

    pub fn equipped_item(&self, slot_id: &str) -> Option<&str> {
        self.equipped.get(slot_id).map(String::as_str)
    }

    pub fn apply_stat_modifiers<'a>(
        &self,
        stats: &mut StatBlock,
        definitions: impl IntoIterator<Item = &'a EquipmentDefinition>,
    ) {
        let definitions = definitions
            .into_iter()
            .map(|definition| (definition.item_id(), definition))
            .collect::<BTreeMap<_, _>>();

        for item_id in self.equipped.values() {
            let Some(definition) = definitions.get(item_id.as_str()) else {
                continue;
            };
            for modifier in definition.flat_modifiers() {
                stats.add_flat_modifier(
                    modifier.stat_id(),
                    format!("equipment:{item_id}"),
                    modifier.value(),
                );
            }
        }
    }
}
