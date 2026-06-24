# Inventory Snippets

These snippets are wiki-ready examples backed by integration tests and the baseplate debug world.

## Create An Inventory

```rust
use openrpg_core::Inventory;

let mut inventory = Inventory::new();
let remainder = inventory.add_stack("mygame:potion_small", 120, 99)?;

assert_eq!(inventory.quantity("mygame:potion_small"), 99);
assert_eq!(remainder, 21);
# Ok::<(), openrpg_core::EngineError>(())
```

## Remove Items

```rust
use openrpg_core::Inventory;

let mut inventory = Inventory::new();
inventory.add_stack("mygame:potion_small", 5, 99)?;
inventory.remove("mygame:potion_small", 3)?;

assert_eq!(inventory.quantity("mygame:potion_small"), 2);
# Ok::<(), openrpg_core::EngineError>(())
```

## Attach Inventory To An Entity

```rust
use openrpg_core::{ComponentSchema, EngineConfig, EntityId, Inventory, OpenRpgCore};

let mut engine = OpenRpgCore::new(EngineConfig::default());
engine.components_mut().define(ComponentSchema::new("core:inventory"));

let mut inventory = Inventory::new();
inventory.add_stack("mygame:potion_small", 3, 99)?;

let hero_id = engine
    .entities_mut()
    .create_with_component(
        EntityId::new("entity:hero"),
        "core:inventory",
        serde_json::to_value(&inventory)?,
    )?;

let entities = engine.entities();
let hero = entities.get(&hero_id).unwrap();
assert!(hero.component("core:inventory").is_some());
# Ok::<(), Box<dyn std::error::Error>>(())
```
