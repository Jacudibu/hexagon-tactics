use bevy_egui::egui::{RichText, Ui};
use game_common::game_data::unit_definition::UnitDefinition;
use game_common::game_data::GameData;

pub fn print_unit_definition_info(ui: &mut Ui, unit: &UnitDefinition, game_data: &GameData) {
    let mut lines = Vec::new();
    let stats = unit.calculate_stats(game_data);
    lines.push(format!("HP: {}", "TODO"));
    lines.push(format!("Move: {} | Jump: {}", stats.movement, stats.jump));
    lines.push(format!("Speed: {}", stats.speed));
    ui.label(lines.join("\n"));

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
}
