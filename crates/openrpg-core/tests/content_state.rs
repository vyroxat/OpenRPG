use openrpg_core::{
    ComponentSchema, ContentEntry, EngineConfig, EntityId, NamespacedId, OpenRpgCore,
};
use serde_json::json;

#[test]
fn namespaced_ids_validate_namespace_and_value() {
    let id = NamespacedId::parse("mygame:iron_sword").expect("valid namespaced id");

    assert_eq!(id.namespace(), "mygame");
    assert_eq!(id.value(), "iron_sword");
    assert_eq!(id.as_str(), "mygame:iron_sword");

    let error = NamespacedId::parse("iron_sword").expect_err("missing namespace should fail");
    assert_eq!(error.code(), "INVALID_NAMESPACED_ID");
}

#[test]
fn content_registry_rejects_duplicate_ids() {
    let mut engine = OpenRpgCore::new(EngineConfig::default());
    engine
        .content_mut()
        .insert(ContentEntry::new(
            "item",
            "mygame:potion_small",
            json!({ "max_stack": 99 }),
        ))
        .expect("first content entry should register");

    let error = engine
        .content_mut()
        .insert(ContentEntry::new(
            "item",
            "mygame:potion_small",
            json!({ "max_stack": 10 }),
        ))
        .expect_err("duplicate content ids should fail");

    assert_eq!(error.code(), "CONTENT_ID_DUPLICATE");
}

#[test]
fn content_registry_rejects_invalid_namespaced_ids() {
    let mut engine = OpenRpgCore::new(EngineConfig::default());

    let error = engine
        .content_mut()
        .insert(ContentEntry::new("item", "potion_small", json!({})))
        .expect_err("content ids must be namespaced");

    assert_eq!(error.code(), "INVALID_NAMESPACED_ID");
}

#[test]
fn content_registry_exposes_entries_by_id_and_kind() {
    let mut engine = OpenRpgCore::new(EngineConfig::default());
    engine
        .content_mut()
        .insert(ContentEntry::new(
            "item",
            "mygame:potion_small",
            json!({ "max_stack": 99 }),
        ))
        .expect("item should register");
    engine
        .content_mut()
        .insert(ContentEntry::new(
            "ability",
            "mygame:firebolt",
            json!({ "cost": { "mana": 8 } }),
        ))
        .expect("ability should register");

    let content = engine.content();
    let potion = content
        .get("mygame:potion_small")
        .expect("potion should be queryable");
    assert_eq!(potion.kind(), "item");
    assert_eq!(potion.data()["max_stack"], 99);

    let item_ids = content.ids_for_kind("item");
    assert_eq!(item_ids, vec!["mygame:potion_small"]);
}

#[test]
fn state_snapshots_restore_entities_world_values_and_tick() {
    let mut engine = OpenRpgCore::new(EngineConfig::default());
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
        .expect("entity should be created");
    engine.set_world_value("tutorial.done", json!(true));
    engine.tick(16);

    let snapshot = engine.snapshot().expect("state should serialize");
    let mut restored = OpenRpgCore::new(EngineConfig::default());
    restored
        .restore_snapshot(snapshot)
        .expect("snapshot should restore");

    assert_eq!(restored.current_tick(), 1);
    assert_eq!(restored.world_value("tutorial.done"), Some(&json!(true)));

    let hero_id = EntityId::new("entity:hero");
    let entities = restored.entities();
    let hero = entities.get(&hero_id).expect("hero should restore");
    assert_eq!(
        hero.component("core:identity").unwrap()["name_key"],
        "character.hero.name"
    );
}

#[test]
fn state_snapshots_roundtrip_through_json() {
    let mut engine = OpenRpgCore::new(EngineConfig::default());
    engine
        .content_mut()
        .insert(ContentEntry::new(
            "item",
            "mygame:potion_small",
            json!({ "max_stack": 99 }),
        ))
        .expect("content should register");
    engine.set_world_value("tutorial.done", json!(true));
    engine.tick(16);

    let encoded = serde_json::to_string(&engine.snapshot().expect("snapshot")).expect("json");
    let decoded = serde_json::from_str(&encoded).expect("snapshot json should decode");

    let mut restored = OpenRpgCore::new(EngineConfig::default());
    restored
        .restore_snapshot(decoded)
        .expect("decoded snapshot should restore");

    assert_eq!(restored.current_tick(), 1);
    assert_eq!(restored.world_value("tutorial.done"), Some(&json!(true)));
    assert_eq!(
        restored
            .content()
            .get("mygame:potion_small")
            .unwrap()
            .data()["max_stack"],
        99
    );
}
