use bevy_egui::egui::Ui;
use game_common::game_data::unit_definition::UnitDefinition;
use game_common::game_data::GameData;

pub fn print_unit_definition_info(ui: &mut Ui, unit: &UnitDefinition, game_data: &GameData) {
    let mut lines = Vec::new();
    let stats = unit.calculate_stats(game_data);
    lines.push(format!("HP: {}", "TODO"));
    lines.push(format!("Move: {} | Jump: {}", stats.movement, stats.jump));
    lines.push(format!("Speed: {}", stats.speed));
    ui.label(lines.join("\n"));
}
