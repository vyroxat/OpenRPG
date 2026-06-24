use openrpg_core::{
    ComponentSchema, EngineConfig, EntityId, Equipment, EquipmentDefinition, OpenRpgCore, StatBlock,
};

#[test]
fn equipment_equips_items_into_allowed_slots() {
    let sword = EquipmentDefinition::new("mygame:iron_sword", ["main_hand"]);
    let mut equipment = Equipment::new();
    equipment.define_slot("main_hand");

    let previous = equipment
        .equip(&sword, "main_hand")
        .expect("sword should equip");

    assert_eq!(previous, None);
    assert_eq!(
        equipment.equipped_item("main_hand"),
        Some("mygame:iron_sword")
    );
}

#[test]
fn equipment_rejects_unknown_or_disallowed_slots() {
    let sword = EquipmentDefinition::new("mygame:iron_sword", ["main_hand"]);
    let mut equipment = Equipment::new();
    equipment.define_slot("off_hand");

    let error = equipment
        .equip(&sword, "main_hand")
        .expect_err("unknown slot should fail");
    assert_eq!(error.code(), "EQUIPMENT_SLOT_UNKNOWN");

    let error = equipment
        .equip(&sword, "off_hand")
        .expect_err("disallowed slot should fail");
    assert_eq!(error.code(), "EQUIPMENT_SLOT_INVALID");
}

#[test]
fn equipment_stat_modifiers_apply_to_stat_blocks() {
    let sword = EquipmentDefinition::new("mygame:iron_sword", ["main_hand"])
        .with_flat_modifier("core:strength", 3.0);
    let mut equipment = Equipment::new();
    equipment.define_slot("main_hand");
    equipment.equip(&sword, "main_hand").expect("sword equips");

    let mut stats = StatBlock::new();
    stats.define_stat("core:strength", 10.0);
    equipment.apply_stat_modifiers(&mut stats, [&sword]);

    assert_eq!(stats.final_value("core:strength"), Some(13.0));
}

#[test]
fn equipment_components_attach_to_entities() {
    let sword = EquipmentDefinition::new("mygame:iron_sword", ["main_hand"]);
    let mut equipment = Equipment::new();
    equipment.define_slot("main_hand");
    equipment.equip(&sword, "main_hand").expect("sword equips");

    let mut engine = OpenRpgCore::new(EngineConfig::default());
    engine
        .components_mut()
        .define(ComponentSchema::new("core:equipment"));

    let hero_id = engine
        .entities_mut()
        .create_with_component(
            EntityId::new("entity:hero"),
            "core:equipment",
            serde_json::to_value(&equipment).expect("equipment serializes"),
        )
        .expect("hero should be created");

    let entities = engine.entities();
    let hero = entities.get(&hero_id).expect("hero exists");
    let restored: Equipment =
        serde_json::from_value(hero.component("core:equipment").unwrap().clone()).unwrap();

    assert_eq!(
        restored.equipped_item("main_hand"),
        Some("mygame:iron_sword")
    );
}
