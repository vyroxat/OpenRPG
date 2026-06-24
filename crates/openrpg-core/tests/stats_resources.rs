use openrpg_core::{ComponentSchema, EngineConfig, EntityId, OpenRpgCore, ResourcePool, StatBlock};

#[test]
fn stat_block_resolves_flat_and_percent_modifiers_with_bounds() {
    let mut stats = StatBlock::new();
    stats.define_stat("core:strength", 10.0);
    stats.set_bounds("core:strength", Some(0.0), Some(20.0));
    stats.add_flat_modifier("core:strength", "mygame:ring", 4.0);
    stats.add_percent_modifier("core:strength", "mygame:blessing", 0.5);

    assert_eq!(stats.final_value("core:strength"), Some(20.0));
    assert_eq!(stats.base_value("core:strength"), Some(10.0));
}

#[test]
fn resource_pool_spends_restores_and_clamps_values() {
    let mut resources = ResourcePool::new();
    resources.define("core:health", 100.0, 80.0);

    resources
        .spend("core:health", 30.0)
        .expect("spend succeeds");
    assert_eq!(resources.current("core:health"), Some(50.0));

    resources
        .restore("core:health", 75.0)
        .expect("restore succeeds");
    assert_eq!(resources.current("core:health"), Some(100.0));

    let error = resources
        .spend("core:health", 101.0)
        .expect_err("overspend should fail");
    assert_eq!(error.code(), "RESOURCE_INSUFFICIENT");
}

#[test]
fn stat_and_resource_components_attach_to_entities() {
    let mut engine = OpenRpgCore::new(EngineConfig::default());
    engine
        .components_mut()
        .define(ComponentSchema::new("core:stats"));
    engine
        .components_mut()
        .define(ComponentSchema::new("core:resources"));

    let mut stats = StatBlock::new();
    stats.define_stat("core:strength", 10.0);
    stats.add_flat_modifier("core:strength", "mygame:ring", 2.0);

    let mut resources = ResourcePool::new();
    resources.define("core:health", 100.0, 80.0);

    let hero_id = engine
        .entities_mut()
        .create_with_component(
            EntityId::new("entity:hero"),
            "core:stats",
            serde_json::to_value(&stats).expect("stats serialize"),
        )
        .expect("hero should be created");
    engine
        .entities_mut()
        .set_component(
            &hero_id,
            "core:resources",
            serde_json::to_value(&resources).expect("resources serialize"),
        )
        .expect("resources should attach");

    let entities = engine.entities();
    let hero = entities.get(&hero_id).expect("hero exists");
    let restored_stats: StatBlock =
        serde_json::from_value(hero.component("core:stats").unwrap().clone()).unwrap();
    let restored_resources: ResourcePool =
        serde_json::from_value(hero.component("core:resources").unwrap().clone()).unwrap();

    assert_eq!(restored_stats.final_value("core:strength"), Some(12.0));
    assert_eq!(restored_resources.current("core:health"), Some(80.0));

    let patches = engine.drain_patches();
    assert!(
        patches
            .iter()
            .any(|patch| patch.path() == "/entities/entity:hero/components/core:resources")
    );
}
