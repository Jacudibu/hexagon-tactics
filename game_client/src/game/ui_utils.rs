use bevy_egui::egui::{RichText, Ui};
use game_common::game_data::unit_definition::UnitDefinition;
use game_common::game_data::GameData;

pub fn print_unit_definition_info(ui: &mut Ui, unit: &UnitDefinition, game_data: &GameData) {
    let race = &game_data.races[&unit.race];
    ui.label(format!("{}", race.name));

    for (class_id, level) in &unit.levels {
        let class = &game_data.classes[class_id];
        ui.label(format!("{} Lv. {}", class.name, level.level));
    }

    ui.label(RichText::new("Stats").heading());

    let stats = unit.calculate_stats(game_data);
    ui.label(format!("HP: {}", stats.max_health));
    ui.label(format!("MP: {}", stats.max_mana));
    ui.label(format!("Move: {} | Jump: {}", stats.movement, stats.jump));
    ui.label(format!("Strength: {}", stats.strength));
    ui.label(format!("Speed: {}", stats.speed));

    ui.label(RichText::new("Equipment").heading());
    if let Some(accessory) = unit.accessory {
        let item = &game_data.accessories[&accessory];
        ui.label(format!("Accessory: {}", item.name));
    } else {
        ui.label("Accessory: None".to_string());
    }
    if let Some(armor) = unit.armor {
        let item = &game_data.armor[&armor];
        ui.label(format!("Armor: {}", item.name));
    } else {
        ui.label("Armor: None".to_string());
    }
    if let Some(weapon) = unit.weapon {
        let item = &game_data.weapons[&weapon];
        ui.label(format!("Weapon: {}", item.name));
    } else {
        ui.label("Weapon: None".to_string());
    }

    ui.label(RichText::new("Skills").heading());
    for skill in unit.all_available_skills(game_data) {
        let skill = &game_data.skills[&skill];
        ui.label(skill.name.clone());
    }
}
