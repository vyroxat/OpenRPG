use openrpg_core::{
    Command, CommandOutcome, CommandResult, ComponentSchema, EngineConfig, EntityId,
    ModuleDescriptor, OpenRpgCore,
};
use serde_json::json;

fn create_engine() -> OpenRpgCore {
    let mut engine = OpenRpgCore::new(EngineConfig::default().with_seed(7));
    engine.register_module(ModuleDescriptor::new("openrpg-core", "0.1.0"));
    engine.boot().expect("core module should boot");
    engine
}

fn load_content(engine: &mut OpenRpgCore) {
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
}

fn create_hero(engine: &mut OpenRpgCore) -> EntityId {
    engine
        .components_mut()
        .define(ComponentSchema::new("core:identity"));
    engine
        .entities_mut()
        .create_with_component(
            EntityId::new("entity:hero"),
            "core:identity",
            json!({ "name_key": "character.hero.name" }),
        )
        .expect("hero entity should be created")
}

fn register_frontend_command(engine: &mut OpenRpgCore) {
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

fn send_frontend_command(engine: &mut OpenRpgCore) {
    let result = engine
        .execute(Command::new(
            "world.setFlag",
            json!({ "key": "tutorial.done", "value": true }),
        ))
        .expect("registered command should execute");
    assert!(matches!(result, CommandResult::Success(_)));
}

fn main() {
    let mut engine = create_engine();
    load_content(&mut engine);
    let hero_id = create_hero(&mut engine);
    register_frontend_command(&mut engine);
    send_frontend_command(&mut engine);

    let frame = engine.tick(16);
    println!("tick: {}", frame.tick());
    println!("events for frontend: {}", frame.events().len());
    println!("patches for frontend: {}", frame.patches().len());
    println!("hero: {hero_id}");
    println!(
        "potion max stack: {}",
        engine.content().get("mygame:potion_small").unwrap().data()["max_stack"]
    );
}
