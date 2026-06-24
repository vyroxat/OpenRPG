# Equipment Snippets

These snippets are wiki-ready examples backed by integration tests and the baseplate debug world.

## Define And Equip An Item

```rust
use openrpg_core::{Equipment, EquipmentDefinition};

let sword = EquipmentDefinition::new("mygame:iron_sword", ["main_hand"]);
let mut equipment = Equipment::new();
equipment.define_slot("main_hand");
equipment.equip(&sword, "main_hand")?;

assert_eq!(equipment.equipped_item("main_hand"), Some("mygame:iron_sword"));
# Ok::<(), openrpg_core::EngineError>(())
```

## Apply Equipment Stat Modifiers

```rust
use openrpg_core::{Equipment, EquipmentDefinition, StatBlock};

let sword = EquipmentDefinition::new("mygame:iron_sword", ["main_hand"])
    .with_flat_modifier("core:strength", 3.0);
let mut equipment = Equipment::new();
equipment.define_slot("main_hand");
equipment.equip(&sword, "main_hand")?;

let mut stats = StatBlock::new();
stats.define_stat("core:strength", 10.0);
equipment.apply_stat_modifiers(&mut stats, [&sword]);

assert_eq!(stats.final_value("core:strength"), Some(13.0));
# Ok::<(), openrpg_core::EngineError>(())
```

## Attach Equipment To An Entity

```rust
use openrpg_core::{
    ComponentSchema, EngineConfig, EntityId, Equipment, EquipmentDefinition, OpenRpgCore,
};

let sword = EquipmentDefinition::new("mygame:iron_sword", ["main_hand"]);
let mut equipment = Equipment::new();
equipment.define_slot("main_hand");
equipment.equip(&sword, "main_hand")?;

let mut engine = OpenRpgCore::new(EngineConfig::default());
engine.components_mut().define(ComponentSchema::new("core:equipment"));

let hero_id = engine
    .entities_mut()
    .create_with_component(
        EntityId::new("entity:hero"),
        "core:equipment",
        serde_json::to_value(&equipment)?,
    )?;

let entities = engine.entities();
let hero = entities.get(&hero_id).unwrap();
assert!(hero.component("core:equipment").is_some());
# Ok::<(), Box<dyn std::error::Error>>(())
```
