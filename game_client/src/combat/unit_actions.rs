use std::cmp::PartialEq;
use std::ops::Deref;

use bevy::app::App;
use bevy::prelude::{
    error, in_state, on_event, resource_changed_or_removed, resource_exists, Commands, Condition,
    Event, EventReader, EventWriter, IntoSystemConfigs, NextState, Plugin, PreUpdate, Res, ResMut,
    Resource, Update,
};
use hexx::Hex;
use leafwing_input_manager::action_state::ActionState;

use game_common::combat_data::CombatData;
use game_common::game_data::skill::{SkillId, SkillTargeting, DEBUG_SINGLE_TARGET_ATTACK_ID};
use game_common::game_map::GameMap;
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::{client_to_server, server_to_client};
use game_common::DESYNC_TODO_MESSAGE;

use crate::combat::combat_input::CombatAction;
use crate::combat::combat_plugin::CombatState;
use crate::combat::end_turn::EndTurnCommand;
use crate::combat::local_combat_data::LocalCombatData;
use crate::combat::unit_animations::MoveUnitComponent;
use crate::combat_data_resource::CombatDataResource;
use crate::game_data_resource::GameDataResource;
use crate::map::{ActiveUnitHighlights, AttackHighlights, CursorOnTile, RangeHighlights};
use crate::networking::LocalPlayerId;
use crate::ApplicationState;

pub struct UnitActionPlugin;
impl Plugin for UnitActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SetOrToggleActiveUnitActionEvent>();
        app.add_systems(
            PreUpdate,
            (
                change_action_on_input,
                on_set_or_toggle_action.run_if(on_event::<SetOrToggleActiveUnitActionEvent>()),
            )
                .chain()
                .run_if(in_state(ApplicationState::InGame))
                .run_if(in_state(CombatState::ThisPlayerUnitTurn)),
        );
        app.add_systems(
            Update,
            (
                on_active_unit_action_changed
                    .run_if(resource_changed_or_removed::<ActiveUnitAction>()),
                execute_action_on_click
                    .run_if(in_state(ApplicationState::InGame))
                    .run_if(in_state(CombatState::ThisPlayerUnitTurn))
                    .run_if(resource_exists::<ActiveUnitAction>),
                on_move_unit.run_if(on_event::<server_to_client::MoveUnit>()),
                on_use_skill.run_if(on_event::<server_to_client::UseSkill>()),
                update_attack_highlights
                    .run_if(in_state(ApplicationState::InGame))
                    .run_if(
                        resource_changed_or_removed::<ActiveUnitAction>()
                            .or_else(resource_changed_or_removed::<CursorOnTile>()),
                    ),
            ),
        );
    }
}

#[derive(Resource, Eq, PartialEq, Copy, Clone)]
pub enum ActiveUnitAction {
    Move,
    UseSkill(SkillId),
}

#[derive(Event)]
pub struct SetOrToggleActiveUnitActionEvent {
    pub action: ActiveUnitAction,
}

pub fn on_set_or_toggle_action(
    mut commands: Commands,
    current_action: Option<Res<ActiveUnitAction>>,
    mut events: EventReader<SetOrToggleActiveUnitActionEvent>,
) {
    for event in events.read() {
        if let Some(current_action) = &current_action {
            if current_action.deref() == &event.action {
                commands.remove_resource::<ActiveUnitAction>();
                return;
            }
        }

        commands.insert_resource(event.action);
    }
}

pub fn change_action_on_input(
    action_state: Res<ActionState<CombatAction>>,
    mut action_state_change_events: EventWriter<SetOrToggleActiveUnitActionEvent>,
    mut end_turn_events: EventWriter<EndTurnCommand>,
) {
    if action_state.just_pressed(&CombatAction::MoveUnit) {
        action_state_change_events.send(SetOrToggleActiveUnitActionEvent {
            action: ActiveUnitAction::Move,
        });
    } else if action_state.just_pressed(&CombatAction::Attack) {
        action_state_change_events.send(SetOrToggleActiveUnitActionEvent {
            action: ActiveUnitAction::UseSkill(DEBUG_SINGLE_TARGET_ATTACK_ID),
        });
    } else if action_state.just_pressed(&CombatAction::EndTurn) {
        end_turn_events.send(EndTurnCommand {});
    }
}

pub fn on_active_unit_action_changed(
    mut commands: Commands,
    combat_data: Res<CombatDataResource>,
    game_data: Res<GameDataResource>,
    map: Res<GameMap>,
    active_unit_action: Option<Res<ActiveUnitAction>>,
) {
    let Some(active_unit_action) = active_unit_action else {
        commands.remove_resource::<RangeHighlights>();
        return;
    };

    match active_unit_action.deref() {
        ActiveUnitAction::Move => {
            show_movement_range_preview(commands, &combat_data, &map);
        }
        ActiveUnitAction::UseSkill(id) => {
            show_skill_range_preview(id, commands, &combat_data, &map, &game_data);
        }
    }
}

pub fn show_movement_range_preview(
    mut commands: Commands,
    combat_data: &CombatData,
    map: &GameMap,
) {
    let unit = combat_data.current_turn_unit();
    let range = map.field_of_movement(unit, combat_data);

    commands.insert_resource(RangeHighlights { tiles: range })
}

pub fn show_skill_range_preview(
    skill_id: &SkillId,
    mut commands: Commands,
    combat_data: &CombatData,
    map: &GameMap,
    data: &GameDataResource,
) {
    let unit = combat_data.current_turn_unit();
    let skill = &data.skills[skill_id];

    match &skill.targeting {
        SkillTargeting::UserPosition => {
            commands.remove_resource::<RangeHighlights>();
            return;
        }
        SkillTargeting::MouseCursor(range) => {
            let tiles: Vec<Hex> = hexx::algorithms::range_fov(unit.position, range.max, |hex| {
                !map.tiles.contains_key(&hex)
            })
            .into_iter()
            .filter(|x| x.unsigned_distance_to(unit.position) >= range.min)
            .collect();

            commands.insert_resource(RangeHighlights { tiles })
        }
    }
}

pub fn execute_action_on_click(
    mut commands: Commands,
    combat_data: Res<CombatDataResource>,
    map: Res<GameMap>,
    action_state: Res<ActionState<CombatAction>>,
    active_unit_action: Res<ActiveUnitAction>,
    range_highlights: Option<Res<RangeHighlights>>,
    mouse_cursor_on_tile: Option<Res<CursorOnTile>>,
    mut event_writer: EventWriter<ClientToServerMessage>,
    mut next_combat_state: ResMut<NextState<CombatState>>,
    game_data: Res<GameDataResource>,
) {
    if !action_state.just_pressed(&CombatAction::SelectTile) {
        return;
    }

    let Some(mouse_cursor_on_tile) = mouse_cursor_on_tile else {
        return;
    };

    let selected_tile = &mouse_cursor_on_tile.hex;
    match active_unit_action.deref() {
        ActiveUnitAction::Move => {
            let Some(range_highlights) = range_highlights else {
                return;
            };

            if !range_highlights.tiles.contains(selected_tile) {
                return;
            }

            // TODO: Path should already exist somewhere for highlighting/preview
            let Some(path) = map.calculate_path(&combat_data, selected_tile.clone()) else {
                error!(
                    "Unable to calculate unit path for {:?} to {:?}",
                    combat_data.current_turn.as_unit_turn(),
                    selected_tile
                );
                return;
            };
            event_writer.send(ClientToServerMessage::MoveUnit(
                client_to_server::MoveUnit { path },
            ));
        }
        ActiveUnitAction::UseSkill(id) => {
            let skill = &game_data.skills[id];
            if skill.targeting != SkillTargeting::UserPosition {
                let Some(range_highlights) = range_highlights else {
                    return;
                };

                if !range_highlights.tiles.contains(selected_tile) {
                    return;
                }
            }

            event_writer.send(ClientToServerMessage::UseSkill(
                client_to_server::UseSkill {
                    id: id.clone(),
                    target_coordinates: selected_tile.clone(),
                },
            ));
        }
    }

    next_combat_state.set(CombatState::WaitingForServer);
    commands.remove_resource::<ActiveUnitAction>(); // TODO: Maybe we should extract that into a state transition event
}

pub fn on_move_unit(
    mut commands: Commands,
    mut events: EventReader<server_to_client::MoveUnit>,
    mut combat_data: ResMut<CombatDataResource>,
    local_combat_data: Res<LocalCombatData>,
    mut next_combat_state: ResMut<NextState<CombatState>>,
    local_player_id: Res<LocalPlayerId>,
) {
    for event in events.read() {
        debug_assert!(
            event.path.len() >= 2,
            "Should contain at least start and end!"
        );

        let old_pos = &event.path[0];
        let new_pos = &event.path[event.path.len() - 1];
        let unit_id = combat_data.current_turn.as_unit_turn().unwrap().unit_id;

        combat_data.unit_positions.remove(old_pos);
        combat_data.unit_positions.insert(new_pos.clone(), unit_id);
        combat_data
            .current_turn
            .as_unit_turn_mut()
            .unwrap()
            .remaining_movement -= event.path.len() as u8 - 1;

        let unit = combat_data
            .units
            .get_mut(&unit_id)
            .expect(DESYNC_TODO_MESSAGE);

        unit.position = new_pos.clone();

        let entity = local_combat_data.unit_entities[&unit_id];

        commands
            .entity(entity)
            .insert(MoveUnitComponent::new(event.path.clone()));

        commands.insert_resource(ActiveUnitHighlights {
            tile: unit.position,
        });

        if unit.owner == local_player_id.id {
            next_combat_state.set(CombatState::ThisPlayerUnitTurn);
        } else {
            next_combat_state.set(CombatState::WaitingForOtherPlayer);
        }
    }
}

pub fn on_use_skill(
    mut events: EventReader<server_to_client::UseSkill>,
    mut combat_data: ResMut<CombatDataResource>,
    mut next_combat_state: ResMut<NextState<CombatState>>,
    local_player_id: Res<LocalPlayerId>,
) {
    for event in events.read() {
        for x in &event.hits {
            let target = combat_data.units.get_mut(&x.target_unit_id).unwrap();

            // TODO: Stat reduction should be extracted into common
            if target.hp < x.physical_damage {
                target.hp = 0;
            } else {
                target.hp -= x.physical_damage;
            }
        }

        // TODO: Animate

        let turn = combat_data.current_turn.as_unit_turn_mut().unwrap();
        turn.remaining_actions -= 1;
        let unit_id = turn.unit_id;

        let unit = combat_data.units.get(&unit_id).unwrap();

        if unit.owner == local_player_id.id {
            next_combat_state.set(CombatState::ThisPlayerUnitTurn);
        } else {
            next_combat_state.set(CombatState::WaitingForOtherPlayer);
        }
    }
}

pub fn update_attack_highlights(
    mut commands: Commands,
    active_unit_action: Option<Res<ActiveUnitAction>>,
    mouse_cursor_on_tile: Option<Res<CursorOnTile>>,
    map: Res<GameMap>,
    combat_data: Res<CombatDataResource>,
    game_data: Res<GameDataResource>,
) {
    let Some(active_unit_action) = active_unit_action else {
        commands.remove_resource::<AttackHighlights>();
        return;
    };

    let Some(mouse_cursor_on_tile) = mouse_cursor_on_tile else {
        commands.remove_resource::<AttackHighlights>();
        return;
    };

    let user_pos = combat_data.current_turn_unit().position;

    match active_unit_action.deref() {
        ActiveUnitAction::Move => commands.remove_resource::<AttackHighlights>(),
        ActiveUnitAction::UseSkill(skill_id) => {
            let skill = &game_data.skills[skill_id];
            commands.insert_resource(AttackHighlights {
                tiles: skill.get_valid_target_hexagons(mouse_cursor_on_tile.hex, user_pos, &map),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::app::App;
    use hexx::Hex;

    use game_common::combat_data::CombatData;
    use game_common::game_data::GameData;
    use game_common::game_map::GameMap;
    use game_common::unit::Unit;

    use crate::combat::unit_actions::{ActiveUnitAction, UnitActionPlugin};
    use crate::combat_data_resource::CombatDataResource;
    use crate::game_data_resource::GameDataResource;
    use crate::map::RangeHighlights;
    use crate::networking::NetworkPlugin;

    #[test]
    fn should_create_and_remove_highlights() {
        let mut app = App::new();
        app.add_plugins(UnitActionPlugin);
        app.add_plugins(NetworkPlugin);
        app.insert_resource(GameMap::new(1));
        app.insert_resource(GameDataResource::new(GameData::create_mock()));

        let unit_id = 1;
        let unit = Unit::create_mock(unit_id, 1).with_position(Hex::ZERO);

        app.insert_resource(CombatDataResource::new(
            CombatData::create_mock()
                .with_units(vec![unit])
                .with_unit_turn(unit_id),
        ));
        app.insert_resource(ActiveUnitAction::Move);
        app.update();

        assert!(
            app.world.get_resource::<RangeHighlights>().is_some(),
            "HighlightedTiles should have been created!"
        );

        app.world.remove_resource::<ActiveUnitAction>();
        app.update();

        assert!(
            app.world.get_resource::<RangeHighlights>().is_none(),
            "HighlightedTiles should have been removed!"
        );
    }
}
