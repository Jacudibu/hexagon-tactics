use crate::game::combat::combat_plugin::CombatState;
use crate::game::combat::end_turn::EndTurnCommand;
use crate::game::combat::leave_combat::LeaveCombatCommand;
use crate::game::combat::unit_actions::{ActiveUnitAction, SetOrToggleActiveUnitActionEvent};
use crate::game::combat::unit_placement::{CurrentlyPlacedUnit, SwitchToNextUnitEvent};
use crate::game::ui_utils;
use crate::map::{CursorOnTile, MapState};
use crate::{ApplicationState, MouseCursorOverUiState};
use bevy::app::{App, Plugin, Update};
use bevy::prelude::*;
use bevy_egui::egui::{Align2, Pos2, Ui};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use game_common::combat_data::CombatData;
use game_common::combat_turn::CombatTurn;
use game_common::combat_unit::CombatUnit;
use game_common::game_data::GameData;
use game_common::player_resources::PlayerResources;

pub(crate) struct CombatUiPlugin;
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
    mut egui: EguiContexts,
    currently_placed_unit: Res<CurrentlyPlacedUnit>,
    player_resources: Res<PlayerResources>,
    game_data: Res<GameData>,
    mut switch_to_next_unit_events: EventWriter<SwitchToNextUnitEvent>,
) {
    let Some(unit) = player_resources
        .units
        .get(currently_placed_unit.array_index)
    else {
        error!(
            "currently_placed_unit array_index was out of range: {}",
            currently_placed_unit.array_index
        );
        return;
    };

    egui::Window::new(&unit.name)
        .collapsible(false)
        .resizable(false)
        .fixed_pos(Pos2::new(5.0, 0.0))
        .anchor(Align2::LEFT_CENTER, egui::Vec2::ZERO)
        .show(egui.ctx_mut(), |ui| {
            ui_utils::print_unit_definition_info(ui, unit, &game_data);
            ui.horizontal(|ui| {
                if ui.button("<<").clicked() {
                    switch_to_next_unit_events.send(SwitchToNextUnitEvent::Previous);
                }
                if ui.button(">>").clicked() {
                    switch_to_next_unit_events.send(SwitchToNextUnitEvent::Next);
                }
            });
        });
}

fn draw_selected_unit_info(
    egui: EguiContexts,
    cursor: Option<Res<CursorOnTile>>,
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

fn draw_unit_info(mut egui: EguiContexts, unit: &CombatUnit, anchor: Align2) {
    let mut lines = Vec::new();
    lines.push(format!("HP: {}", unit.hp));
    lines.push(format!(
        "Move: {} | Jump: {}",
        unit.stats_after_buffs.movement, unit.stats_after_buffs.jump
    ));
    lines.push(format!("Speed: {}", unit.stats_after_buffs.speed));
    lines.push(format!(
        "TurnCounter: {} [{}]",
        unit.turn_counter, unit.turn_tiebreaker
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
    game_data: Res<GameData>,
    end_turn_event: EventWriter<EndTurnCommand>,
    change_unit_action_event: EventWriter<SetOrToggleActiveUnitActionEvent>,
    leave_combat_event: EventWriter<LeaveCombatCommand>,
) {
    egui::Window::new("State Display Window")
        .collapsible(false)
        .resizable(false)
        .title_bar(false)
        .fixed_pos(Pos2::new(0.0, 0.0))
        .pivot(Align2::CENTER_TOP)
        .anchor(Align2::CENTER_TOP, egui::Vec2::ZERO)
        .show(egui.ctx_mut(), |ui| match combat_state.get() {
            CombatState::Defeated => build_defeated_state_ui(ui, leave_combat_event),
            CombatState::Victory => build_victory_state_ui(ui, leave_combat_event),
            CombatState::WaitingForServer => build_waiting_for_server_state_ui(ui),
            CombatState::WaitingForOtherPlayer => build_waiting_for_player_ui(ui, &combat_data),
            CombatState::PlaceUnit => build_place_unit_state_ui(ui),
            CombatState::ThisPlayerUnitTurn => {
                build_this_player_unit_turn_ui(
                    ui,
                    &combat_data,
                    &game_data,
                    change_unit_action_event,
                    end_turn_event,
                );
            }
        });
}

fn build_waiting_for_server_state_ui(ui: &mut Ui) {
    ui.label("Waiting for Server");
}

fn build_place_unit_state_ui(ui: &mut Ui) {
    ui.label("Place Unit");
}

fn build_victory_state_ui(ui: &mut Ui, mut leave_combat_event: EventWriter<LeaveCombatCommand>) {
    ui.label("Victory!");
    if ui.button("Continue").clicked() {
        leave_combat_event.send(LeaveCombatCommand::proceed());
    }
}

fn build_defeated_state_ui(ui: &mut Ui, mut leave_combat_event: EventWriter<LeaveCombatCommand>) {
    ui.label("Defeated!");
    leave_combat_event.send(LeaveCombatCommand::quit());
}

fn build_waiting_for_player_ui(ui: &mut Ui, combat_data: &CombatData) {
    let text = match &combat_data.current_turn {
        CombatTurn::Undefined => "[Undefined] Waiting for player".into(),
        CombatTurn::PlaceUnit(data) => {
            format!("[PlaceUnit] Waiting for Player: {}", data.player_id)
        }
        CombatTurn::UnitTurn(data) => format!(
            "[UnitTurn] Waiting for Player: TODO (unit id {})",
            data.unit_id
        ),
    };

    ui.label(text);
}

fn build_this_player_unit_turn_ui(
    ui: &mut Ui,
    combat_data: &CombatData,
    game_data: &GameData,
    mut change_unit_action_event: EventWriter<SetOrToggleActiveUnitActionEvent>,
    mut end_turn_event: EventWriter<EndTurnCommand>,
) {
    let unit = combat_data.current_turn_unit();
    ui.label(format!("Your turn: {}", unit.name));
    ui.horizontal(|ui| {
        let turn = combat_data.current_turn.as_unit_turn().unwrap();
        ui.add_enabled_ui(turn.remaining_movement > 0, |ui| {
            if ui.button("Move").clicked() {
                change_unit_action_event.send(SetOrToggleActiveUnitActionEvent {
                    action: ActiveUnitAction::Move,
                });
            }
        });
        ui.add_enabled_ui(turn.remaining_actions > 0, |ui| {
            for skill in unit.all_available_skills(game_data) {
                let skill = &game_data.skills[&skill];
                if ui.button(skill.name.clone()).clicked() {
                    change_unit_action_event.send(SetOrToggleActiveUnitActionEvent {
                        action: ActiveUnitAction::UseSkill(skill.id),
                    });
                }
            }
        });
        if ui.button("End Turn").clicked() {
            end_turn_event.send(EndTurnCommand {});
        }
    });
}
