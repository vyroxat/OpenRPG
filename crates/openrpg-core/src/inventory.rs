use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::engine::EngineError;
use crate::namespaced_id::NamespacedId;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
pub struct ItemStack {
    item_id: String,
    quantity: u32,
}

impl ItemStack {
    pub fn new(item_id: impl Into<String>, quantity: u32) -> Result<Self, EngineError> {
        let item_id = item_id.into();
        NamespacedId::parse(&item_id)?;
        if quantity == 0 {
            return Err(EngineError::validation(
                "INVENTORY_QUANTITY_INVALID",
                "item stack quantity must be greater than zero",
            ));
        }
        Ok(Self { item_id, quantity })
    }

    pub fn item_id(&self) -> &str {
        &self.item_id
    }

    pub fn quantity(&self) -> u32 {
        self.quantity
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
pub struct Inventory {
    stacks: BTreeMap<String, ItemStack>,
}

impl Inventory {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_stack(
        &mut self,
        item_id: impl Into<String>,
        quantity: u32,
        max_stack: u32,
    ) -> Result<u32, EngineError> {
        let item_id = item_id.into();
        NamespacedId::parse(&item_id)?;
        if quantity == 0 || max_stack == 0 {
            return Err(EngineError::validation(
                "INVENTORY_QUANTITY_INVALID",
                "item quantity and max stack must be greater than zero",
            ));
        }

        let current = self.quantity(&item_id);
        let available = max_stack.saturating_sub(current);
        let accepted = quantity.min(available);
        let remainder = quantity - accepted;

        if accepted > 0 {
            self.stacks.insert(
                item_id.clone(),
                ItemStack {
                    item_id,
                    quantity: current + accepted,
                },
            );
        }

        Ok(remainder)
    }

    pub fn remove(&mut self, item_id: &str, quantity: u32) -> Result<(), EngineError> {
        NamespacedId::parse(item_id)?;
        if quantity == 0 {
            return Err(EngineError::validation(
                "INVENTORY_QUANTITY_INVALID",
                "item remove quantity must be greater than zero",
            ));
        }

        let current = self.quantity(item_id);
        if current < quantity {
            return Err(EngineError::validation(
                "INVENTORY_ITEM_INSUFFICIENT",
                format!("item {item_id} has insufficient quantity"),
            ));
        }

        let next = current - quantity;
        if next == 0 {
            self.stacks.remove(item_id);
        } else if let Some(stack) = self.stacks.get_mut(item_id) {
            stack.quantity = next;
        }

        Ok(())
    }

    pub fn quantity(&self, item_id: &str) -> u32 {
        self.stacks
            .get(item_id)
            .map(ItemStack::quantity)
            .unwrap_or_default()
    }

    pub fn stacks(&self) -> impl Iterator<Item = &ItemStack> {
        self.stacks.values()
    }
}
