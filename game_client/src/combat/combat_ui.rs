use crate::combat::combat_plugin::CombatState;
use crate::combat::unit_placement::CurrentlyPlacedUnit;
use crate::map::{MapState, MouseCursorOnTile};
use crate::{ApplicationState, MouseCursorOverUiState};
use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;
use bevy_egui::egui::{Align2, Pos2};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use game_common::combat_data::CombatData;
use game_common::network_events::CONSTANT_LOCAL_PLAYER_ID;
use game_common::unit::Unit;

pub(in crate::combat) struct CombatUiPlugin;
impl Plugin for CombatUiPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }

        app.init_state::<MouseCursorOverUiState>().add_systems(
            Update,
            (
                draw_currently_placed_unit_info
                    .run_if(in_state(CombatState::PlaceUnit))
                    .run_if(resource_exists::<CurrentlyPlacedUnit>),
                (draw_selected_unit_info, draw_state_ui)
                    .run_if(in_state(ApplicationState::InGame))
                    .run_if(in_state(MapState::Loaded)),
            ),
        );
    }
}

fn draw_currently_placed_unit_info(
    egui: EguiContexts,
    currently_placed_unit: Res<CurrentlyPlacedUnit>,
    combat_data: Res<CombatData>,
) {
    let Some(unit) = combat_data
        .unit_storage
        .get(currently_placed_unit.array_index)
    else {
        error!(
            "currently_placed_unit array_index was out of range: {}",
            currently_placed_unit.array_index
        );
        return;
    };

    draw_unit_info(egui, unit, Align2::LEFT_CENTER);
}

fn draw_selected_unit_info(
    egui: EguiContexts,
    cursor: Option<Res<MouseCursorOnTile>>,
    combat_data: Res<CombatData>,
) {
    let Some(cursor) = cursor else {
        return;
    };

    let Some(unit_id) = combat_data.unit_positions.get(&cursor.hex) else {
        return;
    };

    let Some(unit) = combat_data.units.get(unit_id) else {
        error!(
            "Unable to find currently selected unit in unit list! Id: {}",
            unit_id
        );
        return;
    };

    draw_unit_info(egui, unit, Align2::LEFT_BOTTOM);
}

fn draw_unit_info(mut egui: EguiContexts, unit: &Unit, anchor: Align2) {
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
        .anchor(anchor, egui::Vec2::ZERO)
        .show(egui.ctx_mut(), |ui| ui.label(text));
}

fn draw_state_ui(
    mut egui: EguiContexts,
    combat_state: Res<State<CombatState>>,
    combat_data: Res<CombatData>,
) {
    let text = match combat_state.get() {
        CombatState::WaitingForOtherPlayer => {
            format!("Waiting for player {}", CONSTANT_LOCAL_PLAYER_ID)
        }
        CombatState::WaitingForServer => "Waiting for Server".into(),
        CombatState::PlaceUnit => "Place Unit".into(),
        CombatState::ThisPlayerUnitTurn => {
            let unit = combat_data
                .units
                .get(
                    &combat_data
                        .current_unit_turn
                        .expect("current_unit_turn should be set!"),
                )
                .expect("Unit should exist!");
            format!("Your turn: {}", unit.name)
        }
        CombatState::OtherPlayerUnitTurn => {
            let unit = combat_data
                .units
                .get(
                    &combat_data
                        .current_unit_turn
                        .expect("current_unit_turn should be set!"),
                )
                .expect("Unit should exist!");
            format!("{}'s turn: {}", CONSTANT_LOCAL_PLAYER_ID, unit.name)
        }
    };

    egui::Window::new("State Display Window")
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .fixed_pos(Pos2::new(0.0, 0.0))
        .pivot(Align2::CENTER_TOP)
        .anchor(Align2::CENTER_TOP, egui::Vec2::ZERO)
        .show(egui.ctx_mut(), |ui| ui.label(text));
}
