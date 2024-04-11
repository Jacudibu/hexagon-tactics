use crate::combat::unit_placement::CurrentlyPlacedUnit;
use crate::combat::{CombatState, CurrentlySelectedUnit};
use crate::MouseCursorOverUiState;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;
use bevy_egui::egui::{Align2, Pos2};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use game_common::game_state::CombatData;

pub(in crate::combat) struct CombatUiPlugin;
impl Plugin for CombatUiPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }

        app.init_state::<MouseCursorOverUiState>().add_systems(
            Update,
            (draw_unit_info_ui)
                .run_if(in_state(CombatState::PlaceUnit))
                .run_if(resource_exists::<CurrentlyPlacedUnit>),
        );
    }
}

fn draw_unit_info_ui(
    mut egui: EguiContexts,
    unit: Option<Res<CurrentlyPlacedUnit>>,
    combat_data: Res<CombatData>,
) {
    let Some(unit) = unit else {
        error!("draw_unit_info_ui was called when currently_selected_unit was None!");
        return;
    };

    let Some(unit) = combat_data.units.get(&unit.unit_id) else {
        error!(
            "Unable to find currently selected unit in unit list! Id: {}",
            unit.unit_id
        );
        return;
    };

    let mut lines = Vec::new();
    lines.push(format!("HP: {}", unit.hp));
    lines.push(format!(
        "Move: {} | Jump: {}",
        unit.stats_after_buffs.movement, unit.stats_after_buffs.jump
    ));

    let text = lines.join("\n");

    egui::Window::new(&unit.name)
        .collapsible(false)
        .resizable(false)
        .fixed_pos(Pos2::new(5.0, 0.0))
        .anchor(Align2::LEFT_CENTER, egui::Vec2::ZERO)
        .show(egui.ctx_mut(), |ui| ui.label(text));
}
