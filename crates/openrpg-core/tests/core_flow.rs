use openrpg_core::{
    Command, CommandOutcome, CommandResult, ComponentSchema, EngineConfig, EngineError, EntityId,
    InterceptorDecision, ModuleDescriptor, OpenRpgCore, PatchOp,
};
use serde_json::json;

fn demo_module(id: &str) -> ModuleDescriptor {
    ModuleDescriptor::new(id, "0.1.0")
}

#[test]
fn boots_modules_when_dependencies_are_registered() {
    let mut engine = OpenRpgCore::new(EngineConfig::default());
    engine.register_module(demo_module("openrpg-inventory"));
    engine.register_module(
        ModuleDescriptor::new("openrpg-equipment", "0.1.0").requires("openrpg-inventory"),
    );

    engine.boot().expect("module dependencies should be valid");

    assert!(engine.is_booted());
    assert_eq!(
        engine.module_order(),
        vec!["openrpg-inventory", "openrpg-equipment"]
    );
}

#[test]
fn boot_fails_when_required_module_is_missing() {
    let mut engine = OpenRpgCore::new(EngineConfig::default());
    engine.register_module(
        ModuleDescriptor::new("openrpg-equipment", "0.1.0").requires("openrpg-inventory"),
    );

    let error = engine
        .boot()
        .expect_err("missing dependency should fail boot");

    assert_eq!(error.code(), "MODULE_DEPENDENCY_MISSING");
    assert!(error.message().contains("openrpg-inventory"));
}

#[test]
fn creates_entities_with_components_and_records_patch() {
    let mut engine = OpenRpgCore::new(EngineConfig::default());
    engine
        .components_mut()
        .define(ComponentSchema::new("core:identity"));

    let entity_id = engine
        .entities_mut()
        .create_with_component(
            EntityId::new("entity:hero"),
            "core:identity",
            json!({ "name_key": "character.hero.name" }),
        )
        .expect("registered component should be accepted");

    let entities = engine.entities();
    let entity = entities.get(&entity_id).expect("entity exists");
    assert_eq!(entity.id(), &entity_id);
    assert_eq!(
        entity.component("core:identity").unwrap()["name_key"],
        "character.hero.name"
    );

    let patches = engine.drain_patches();
    assert_eq!(patches.len(), 1);
    assert_eq!(patches[0].op(), PatchOp::Add);
    assert_eq!(patches[0].path(), "/entities/entity:hero");
}

#[test]
fn registered_command_can_emit_events_and_patches() {
    let mut engine = OpenRpgCore::new(EngineConfig::default());
    engine
        .commands_mut()
        .register("world.setFlag", |ctx, command| {
            let key = command.payload()["key"].as_str().ok_or_else(|| {
                EngineError::validation("INVALID_FLAG_KEY", "flag key is required")
            })?;
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
        .expect("known command should execute");

    assert!(matches!(result, CommandResult::Success(_)));
    assert_eq!(engine.world_value("tutorial.done"), Some(&json!(true)));

    let tick = engine.tick(16);
    assert_eq!(tick.tick(), 1);
    assert_eq!(tick.events().len(), 1);
    assert_eq!(tick.events()[0].event_type(), "world.flagSet");
    assert_eq!(tick.patches()[0].path(), "/world/tutorial.done");
}

#[test]
fn interceptors_can_deny_commands_before_handlers_run() {
    let mut engine = OpenRpgCore::new(EngineConfig::default());
    engine
        .commands_mut()
        .register("debug.secret", |_ctx, _command| {
            Ok(CommandOutcome::default())
        });
    engine
        .interceptors_mut()
        .before_command("debug.secret", |_command| {
            InterceptorDecision::deny(EngineError::forbidden(
                "COMMAND_FORBIDDEN",
                "secret command denied",
            ))
        });

    let error = engine
        .execute(Command::new("debug.secret", json!({})))
        .expect_err("interceptor should deny command");

    assert_eq!(error.code(), "COMMAND_FORBIDDEN");
}

#[test]
fn seeded_rng_produces_stable_sequences() {
    let mut first = OpenRpgCore::new(EngineConfig::default().with_seed(42));
    let mut second = OpenRpgCore::new(EngineConfig::default().with_seed(42));

    let first_rolls = [
        first.rng_mut().next_u32(),
        first.rng_mut().next_u32(),
        first.rng_mut().range_u32(1..=20),
    ];
    let second_rolls = [
        second.rng_mut().next_u32(),
        second.rng_mut().next_u32(),
        second.rng_mut().range_u32(1..=20),
    ];

    assert_eq!(first_rolls, second_rolls);
}
