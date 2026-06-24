use openrpg_core::{
    Command, CommandOutcome, CommandResult, ComponentSchema, EngineConfig, EntityId, Equipment,
    EquipmentDefinition, Inventory, ModuleDescriptor, OpenRpgCore, ResourcePool, StatBlock,
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

fn create_hero(engine: &mut OpenRpgCore) -> EntityId {
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

fn attach_mechanics(engine: &mut OpenRpgCore, hero_id: &EntityId) {
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
    attach_mechanics(&mut engine, &hero_id);
    register_frontend_command(&mut engine);
    send_frontend_command(&mut engine);

    let entities = engine.entities();
    let hero = entities.get(&hero_id).expect("hero exists");
    let stats: StatBlock =
        serde_json::from_value(hero.component("core:stats").unwrap().clone()).expect("stats");
    let resources: ResourcePool =
        serde_json::from_value(hero.component("core:resources").unwrap().clone())
            .expect("resources");
    let inventory: Inventory =
        serde_json::from_value(hero.component("core:inventory").unwrap().clone())
            .expect("inventory");
    let equipment: Equipment =
        serde_json::from_value(hero.component("core:equipment").unwrap().clone())
            .expect("equipment");

    let frame = engine.tick(16);
    println!("tick: {}", frame.tick());
    println!("events for frontend: {}", frame.events().len());
    println!("patches for frontend: {}", frame.patches().len());
    println!("hero: {hero_id}");
    println!(
        "hero strength: {}",
        stats.final_value("core:strength").unwrap()
    );
    println!("hero health: {}", resources.current("core:health").unwrap());
    println!(
        "hero potions: {}",
        inventory.quantity("mygame:potion_small")
    );
    println!(
        "hero main hand: {}",
        equipment.equipped_item("main_hand").unwrap()
    );
    println!(
        "potion max stack: {}",
        engine.content().get("mygame:potion_small").unwrap().data()["max_stack"]
    );
}
