use openrpg_core::{
    Command, CommandOutcome, CommandResult, ComponentSchema, EngineConfig, EntityId,
    ModuleDescriptor, OpenRpgCore,
};
use serde_json::json;

#[test]
fn baseplate_core_flow_runs_with_content_entities_commands_ticks_and_save_restore() {
    let mut engine = OpenRpgCore::new(EngineConfig::default().with_seed(7));
    engine.register_module(ModuleDescriptor::new("openrpg-core", "0.1.0"));
    engine.boot().expect("core module should boot");

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
                  "kind": "ability",
                  "id": "mygame:firebolt",
                  "data": { "cost": { "mana": 8 }, "damage": 12 }
                }
              ]
            }
            "#,
        )
        .expect("baseplate content should load");

    engine
        .components_mut()
        .define(ComponentSchema::new("core:identity"));
    let hero_id = engine
        .entities_mut()
        .create_with_component(
            EntityId::new("entity:hero"),
            "core:identity",
            json!({ "name_key": "character.hero.name" }),
        )
        .expect("hero entity should be created");

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
}
