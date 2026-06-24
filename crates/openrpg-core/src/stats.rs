use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::engine::EngineError;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum StatModifierKind {
    Flat,
    Percent,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatModifier {
    source: String,
    kind: StatModifierKind,
    value: f64,
}

impl StatModifier {
    pub fn flat(source: impl Into<String>, value: f64) -> Self {
        Self {
            source: source.into(),
            kind: StatModifierKind::Flat,
            value,
        }
    }

    pub fn percent(source: impl Into<String>, value: f64) -> Self {
        Self {
            source: source.into(),
            kind: StatModifierKind::Percent,
            value,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct StatValue {
    base: f64,
    floor: Option<f64>,
    cap: Option<f64>,
    modifiers: Vec<StatModifier>,
}

impl StatValue {
    fn new(base: f64) -> Self {
        Self {
            base,
            floor: None,
            cap: None,
            modifiers: Vec::new(),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct StatBlock {
    stats: BTreeMap<String, StatValue>,
}

impl StatBlock {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define_stat(&mut self, id: impl Into<String>, base: f64) {
        self.stats.insert(id.into(), StatValue::new(base));
    }

    pub fn set_bounds(&mut self, id: &str, floor: Option<f64>, cap: Option<f64>) {
        if let Some(stat) = self.stats.get_mut(id) {
            stat.floor = floor;
            stat.cap = cap;
        }
    }

    pub fn add_flat_modifier(&mut self, id: &str, source: impl Into<String>, value: f64) {
        if let Some(stat) = self.stats.get_mut(id) {
            stat.modifiers.push(StatModifier::flat(source, value));
        }
    }

    pub fn add_percent_modifier(&mut self, id: &str, source: impl Into<String>, value: f64) {
        if let Some(stat) = self.stats.get_mut(id) {
            stat.modifiers.push(StatModifier::percent(source, value));
        }
    }

    pub fn base_value(&self, id: &str) -> Option<f64> {
        self.stats.get(id).map(|stat| stat.base)
    }

    pub fn final_value(&self, id: &str) -> Option<f64> {
        let stat = self.stats.get(id)?;
        let flat_total = stat
            .modifiers
            .iter()
            .filter(|modifier| modifier.kind == StatModifierKind::Flat)
            .map(|modifier| modifier.value)
            .sum::<f64>();
        let percent_total = stat
            .modifiers
            .iter()
            .filter(|modifier| modifier.kind == StatModifierKind::Percent)
            .map(|modifier| modifier.value)
            .sum::<f64>();

        let mut value = (stat.base + flat_total) * (1.0 + percent_total);
        if let Some(floor) = stat.floor {
            value = value.max(floor);
        }
        if let Some(cap) = stat.cap {
            value = value.min(cap);
        }
        Some(value)
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ResourceValue {
    max: f64,
    current: f64,
}

impl ResourceValue {
    fn new(max: f64, current: f64) -> Self {
        Self {
            max,
            current: current.clamp(0.0, max),
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ResourcePool {
    resources: BTreeMap<String, ResourceValue>,
}

impl ResourcePool {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn define(&mut self, id: impl Into<String>, max: f64, current: f64) {
        self.resources
            .insert(id.into(), ResourceValue::new(max, current));
    }

    pub fn current(&self, id: &str) -> Option<f64> {
        self.resources.get(id).map(|resource| resource.current)
    }

    pub fn max(&self, id: &str) -> Option<f64> {
        self.resources.get(id).map(|resource| resource.max)
    }

    pub fn spend(&mut self, id: &str, amount: f64) -> Result<(), EngineError> {
        let resource = self.resource_mut(id)?;
        if amount < 0.0 {
            return Err(EngineError::validation(
                "RESOURCE_AMOUNT_INVALID",
                "resource spend amount must be non-negative",
            ));
        }
        if resource.current < amount {
            return Err(EngineError::validation(
                "RESOURCE_INSUFFICIENT",
                format!("resource {id} has insufficient current value"),
            ));
        }
        resource.current -= amount;
        Ok(())
    }

    pub fn restore(&mut self, id: &str, amount: f64) -> Result<(), EngineError> {
        let resource = self.resource_mut(id)?;
        if amount < 0.0 {
            return Err(EngineError::validation(
                "RESOURCE_AMOUNT_INVALID",
                "resource restore amount must be non-negative",
            ));
        }
        resource.current = (resource.current + amount).min(resource.max);
        Ok(())
    }

    fn resource_mut(&mut self, id: &str) -> Result<&mut ResourceValue, EngineError> {
        self.resources.get_mut(id).ok_or_else(|| {
            EngineError::validation("RESOURCE_UNKNOWN", format!("resource {id} is not defined"))
        })
    }
}
