use openrpg_core::{
    Command, CommandOutcome, CommandResult, ComponentSchema, EngineConfig, EntityId, Equipment,
    EquipmentDefinition, Inventory, ModuleDescriptor, OpenRpgCore, ResourcePool, StatBlock,
};
use serde_json::json;

fn create_baseplate_engine() -> OpenRpgCore {
    let mut engine = OpenRpgCore::new(EngineConfig::default().with_seed(7));
    engine.register_module(ModuleDescriptor::new("openrpg-core", "0.1.0"));
    engine.boot().expect("core module should boot");
    engine
}

fn load_baseplate_content(engine: &mut OpenRpgCore) {
    engine
        .load_content_pack_json(
            r#"
            {
              "entries": [
                {
                  "kind": "item",
                  "id": "mygame:potion_small",
                  "data": { "max_stack": 99, "tags": ["potion", "healing"] }
                },
                {
                  "kind": "equipment",
                  "id": "mygame:iron_sword",
                  "data": {
                    "slots": ["main_hand"],
                    "stat_modifiers": [{ "stat": "core:strength", "flat": 3.0 }]
                  }
                },
                {
                  "kind": "ability",
                  "id": "mygame:firebolt",
                  "data": { "cost": { "mana": 8 }, "damage": 12 }
                }
              ]
            }
            "#,
        )
        .expect("baseplate content should load");
}

fn create_baseplate_hero(engine: &mut OpenRpgCore) -> EntityId {
    engine
        .components_mut()
        .define(ComponentSchema::new("core:identity"));
    engine
        .components_mut()
        .define(ComponentSchema::new("core:stats"));
    engine
        .components_mut()
        .define(ComponentSchema::new("core:resources"));
    engine
        .components_mut()
        .define(ComponentSchema::new("core:inventory"));
    engine
        .components_mut()
        .define(ComponentSchema::new("core:equipment"));

    engine
        .entities_mut()
        .create_with_component(
            EntityId::new("entity:hero"),
            "core:identity",
            json!({ "name_key": "character.hero.name" }),
        )
        .expect("hero entity should be created")
}

fn attach_baseplate_mechanics(engine: &mut OpenRpgCore, hero_id: &EntityId) {
    let mut stats = StatBlock::new();
    stats.define_stat("core:strength", 10.0);
    stats.add_flat_modifier("core:strength", "mygame:ring", 2.0);

    let iron_sword = EquipmentDefinition::new("mygame:iron_sword", ["main_hand"])
        .with_flat_modifier("core:strength", 3.0);
    let mut equipment = Equipment::new();
    equipment.define_slot("main_hand");
    equipment
        .equip(&iron_sword, "main_hand")
        .expect("sword equip");
    equipment.apply_stat_modifiers(&mut stats, [&iron_sword]);

    let mut resources = ResourcePool::new();
    resources.define("core:health", 100.0, 80.0);

    engine
        .entities_mut()
        .set_component(
            hero_id,
            "core:stats",
            serde_json::to_value(&stats).expect("stats serialize"),
        )
        .expect("stats attach");
    engine
        .entities_mut()
        .set_component(
            hero_id,
            "core:resources",
            serde_json::to_value(&resources).expect("resources serialize"),
        )
        .expect("resources attach");

    let potion_max_stack = engine.content().get("mygame:potion_small").unwrap().data()["max_stack"]
        .as_u64()
        .expect("potion max_stack is numeric") as u32;
    let mut inventory = Inventory::new();
    inventory
        .add_stack("mygame:potion_small", 3, potion_max_stack)
        .expect("inventory add");
    engine
        .entities_mut()
        .set_component(
            hero_id,
            "core:inventory",
            serde_json::to_value(&inventory).expect("inventory serialize"),
        )
        .expect("inventory attach");
    engine
        .entities_mut()
        .set_component(
            hero_id,
            "core:equipment",
            serde_json::to_value(&equipment).expect("equipment serialize"),
        )
        .expect("equipment attach");
}

fn register_baseplate_frontend_command(engine: &mut OpenRpgCore) {
    engine
        .commands_mut()
        .register("world.setFlag", |ctx, command| {
            let key = command.payload()["key"]
                .as_str()
                .expect("baseplate command includes key");
            let value = command.payload()["value"].clone();

            ctx.set_world_value(key, value.clone());
            ctx.emit("world.flagSet", json!({ "key": key, "value": value }));

            Ok(CommandOutcome::default())
        });
}

#[test]
fn baseplate_core_flow_runs_with_content_entities_commands_ticks_and_save_restore() {
    let mut engine = create_baseplate_engine();
    load_baseplate_content(&mut engine);
    let hero_id = create_baseplate_hero(&mut engine);
    attach_baseplate_mechanics(&mut engine, &hero_id);
    register_baseplate_frontend_command(&mut engine);

    let result = engine
        .execute(Command::new(
            "world.setFlag",
            json!({ "key": "tutorial.done", "value": true }),
        ))
        .expect("registered command should execute");
    assert!(matches!(result, CommandResult::Success(_)));

    let tick = engine.tick(16);
    assert_eq!(tick.tick(), 1);
    assert_eq!(tick.events()[0].event_type(), "world.flagSet");
    assert_eq!(engine.world_value("tutorial.done"), Some(&json!(true)));
    assert_eq!(engine.rng_mut().range_u32(1..=20), 3);

    let snapshot_json =
        serde_json::to_string(&engine.snapshot().expect("snapshot should serialize"))
            .expect("snapshot should encode as json");
    let snapshot = serde_json::from_str(&snapshot_json).expect("snapshot should decode");
    let mut restored = OpenRpgCore::new(EngineConfig::default());
    restored
        .restore_snapshot(snapshot)
        .expect("snapshot should restore");

    assert_eq!(restored.current_tick(), 1);
    assert_eq!(
        restored
            .content()
            .get("mygame:potion_small")
            .unwrap()
            .data()["max_stack"],
        99
    );
    assert_eq!(
        restored
            .entities()
            .get(&hero_id)
            .unwrap()
            .component("core:identity")
            .unwrap()["name_key"],
        "character.hero.name"
    );

    let restored_stats: StatBlock = serde_json::from_value(
        restored
            .entities()
            .get(&hero_id)
            .unwrap()
            .component("core:stats")
            .unwrap()
            .clone(),
    )
    .expect("stats restore");
    let restored_resources: ResourcePool = serde_json::from_value(
        restored
            .entities()
            .get(&hero_id)
            .unwrap()
            .component("core:resources")
            .unwrap()
            .clone(),
    )
    .expect("resources restore");
    let restored_inventory: Inventory = serde_json::from_value(
        restored
            .entities()
            .get(&hero_id)
            .unwrap()
            .component("core:inventory")
            .unwrap()
            .clone(),
    )
    .expect("inventory restore");
    let restored_equipment: Equipment = serde_json::from_value(
        restored
            .entities()
            .get(&hero_id)
            .unwrap()
            .component("core:equipment")
            .unwrap()
            .clone(),
    )
    .expect("equipment restore");
    assert_eq!(restored_stats.final_value("core:strength"), Some(15.0));
    assert_eq!(restored_resources.current("core:health"), Some(80.0));
    assert_eq!(restored_inventory.quantity("mygame:potion_small"), 3);
    assert_eq!(
        restored_equipment.equipped_item("main_hand"),
        Some("mygame:iron_sword")
    );
}

#[test]
fn baseplate_snippets_cover_current_debug_world_surface() {
    let mut engine = create_baseplate_engine();
    load_baseplate_content(&mut engine);
    let hero_id = create_baseplate_hero(&mut engine);
    attach_baseplate_mechanics(&mut engine, &hero_id);
    register_baseplate_frontend_command(&mut engine);

    assert!(engine.is_booted());
    assert!(engine.content().get("mygame:potion_small").is_some());
    assert!(engine.entities().get(&hero_id).is_some());
    assert!(
        engine
            .entities()
            .get(&hero_id)
            .unwrap()
            .component("core:stats")
            .is_some()
    );
    assert!(
        engine
            .entities()
            .get(&hero_id)
            .unwrap()
            .component("core:inventory")
            .is_some()
    );
    assert!(
        engine
            .entities()
            .get(&hero_id)
            .unwrap()
            .component("core:equipment")
            .is_some()
    );
    assert!(
        engine
            .execute(Command::new(
                "world.setFlag",
                json!({ "key": "debug.snippet", "value": "covered" }),
            ))
            .is_ok()
    );
}
