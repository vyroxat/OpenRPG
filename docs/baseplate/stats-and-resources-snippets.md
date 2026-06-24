# Stats And Resources Snippets

These snippets are wiki-ready examples backed by the baseplate and integration tests.

## Create Stats

```rust
use openrpg_core::StatBlock;

let mut stats = StatBlock::new();
stats.define_stat("core:strength", 10.0);
stats.add_flat_modifier("core:strength", "mygame:ring", 2.0);

assert_eq!(stats.final_value("core:strength"), Some(12.0));
```

## Add Bounds And Percent Modifiers

```rust
use openrpg_core::StatBlock;

let mut stats = StatBlock::new();
stats.define_stat("core:strength", 10.0);
stats.set_bounds("core:strength", Some(0.0), Some(20.0));
stats.add_flat_modifier("core:strength", "mygame:ring", 4.0);
stats.add_percent_modifier("core:strength", "mygame:blessing", 0.5);

assert_eq!(stats.final_value("core:strength"), Some(20.0));
```

## Create Resources

```rust
use openrpg_core::ResourcePool;

let mut resources = ResourcePool::new();
resources.define("core:health", 100.0, 80.0);
resources.spend("core:health", 30.0)?;
resources.restore("core:health", 75.0)?;

assert_eq!(resources.current("core:health"), Some(100.0));
# Ok::<(), openrpg_core::EngineError>(())
```

## Attach To An Entity

```rust
use openrpg_core::{ComponentSchema, EngineConfig, EntityId, OpenRpgCore, ResourcePool, StatBlock};

let mut engine = OpenRpgCore::new(EngineConfig::default());
engine.components_mut().define(ComponentSchema::new("core:stats"));
engine.components_mut().define(ComponentSchema::new("core:resources"));

let mut stats = StatBlock::new();
stats.define_stat("core:strength", 10.0);

let mut resources = ResourcePool::new();
resources.define("core:health", 100.0, 80.0);

let hero_id = engine
    .entities_mut()
    .create_with_component(
        EntityId::new("entity:hero"),
        "core:stats",
        serde_json::to_value(&stats)?,
    )?;

engine.entities_mut().set_component(
    &hero_id,
    "core:resources",
    serde_json::to_value(&resources)?,
)?;

# Ok::<(), Box<dyn std::error::Error>>(())
```
