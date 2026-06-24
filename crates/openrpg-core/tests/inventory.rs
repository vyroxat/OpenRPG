use openrpg_core::{ComponentSchema, EngineConfig, EntityId, Inventory, OpenRpgCore};

#[test]
fn inventory_adds_items_until_stack_limit_and_reports_remainder() {
    let mut inventory = Inventory::new();

    let remainder = inventory
        .add_stack("mygame:potion_small", 120, 99)
        .expect("stack should add");

    assert_eq!(inventory.quantity("mygame:potion_small"), 99);
    assert_eq!(remainder, 21);
}

#[test]
fn inventory_removes_items_and_rejects_overspend() {
    let mut inventory = Inventory::new();
    inventory
        .add_stack("mygame:potion_small", 5, 99)
        .expect("stack should add");

    inventory
        .remove("mygame:potion_small", 3)
        .expect("remove should succeed");
    assert_eq!(inventory.quantity("mygame:potion_small"), 2);

    let error = inventory
        .remove("mygame:potion_small", 3)
        .expect_err("cannot remove more than exists");
    assert_eq!(error.code(), "INVENTORY_ITEM_INSUFFICIENT");
}

#[test]
fn inventory_components_attach_to_entities() {
    let mut engine = OpenRpgCore::new(EngineConfig::default());
    engine
        .components_mut()
        .define(ComponentSchema::new("core:inventory"));

    let mut inventory = Inventory::new();
    inventory
        .add_stack("mygame:potion_small", 3, 99)
        .expect("stack should add");

    let hero_id = engine
        .entities_mut()
        .create_with_component(
            EntityId::new("entity:hero"),
            "core:inventory",
            serde_json::to_value(&inventory).expect("inventory serializes"),
        )
        .expect("hero should be created");

    let entities = engine.entities();
    let hero = entities.get(&hero_id).expect("hero exists");
    let restored: Inventory =
        serde_json::from_value(hero.component("core:inventory").unwrap().clone()).unwrap();

    assert_eq!(restored.quantity("mygame:potion_small"), 3);
}
