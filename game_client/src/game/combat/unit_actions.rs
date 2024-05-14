use std::cmp::PartialEq;
use std::ops::Deref;

use bevy::app::App;
use bevy::prelude::*;
use bevy_sprite3d::Sprite3dParams;
use leafwing_input_manager::action_state::ActionState;

use game_common::combat_data::CombatData;
use game_common::game_data::skill::{SkillId, SkillTargeting, DEBUG_SINGLE_TARGET_ATTACK_ID};
use game_common::game_data::GameData;
use game_common::game_map::GameMap;
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::{client_to_server, server_to_client};
use game_common::DESYNC_TODO_MESSAGE;

use crate::game::combat::combat_input::CombatAction;
use crate::game::combat::combat_plugin::CombatState;
use crate::game::combat::end_turn::EndTurnCommand;
use crate::game::combat::local_combat_data::LocalCombatData;
use crate::game::combat::unit_actions_highlights::UnitActionHighlightPlugin;
use crate::game::combat::unit_animations::{MoveUnitComponent, UnitAttackAnimationComponent};
use crate::game::sprite_builder;
use crate::load::CharacterSprites;
use crate::map::{ActiveUnitHighlights, CursorOnTile, PathHighlights, RangeHighlights};
use crate::networking::LocalPlayerId;
use crate::ApplicationState;

pub struct UnitActionPlugin;
impl Plugin for UnitActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UnitActionHighlightPlugin);
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
                execute_action_on_click
                    .run_if(in_state(ApplicationState::InGame))
                    .run_if(in_state(CombatState::ThisPlayerUnitTurn))
                    .run_if(resource_exists::<ActiveUnitAction>),
                on_move_unit.run_if(on_event::<server_to_client::MoveUnit>()),
                on_use_skill.run_if(on_event::<server_to_client::UseSkill>()),
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

pub fn execute_action_on_click(
    mut commands: Commands,
    action_state: Res<ActionState<CombatAction>>,
    active_unit_action: Res<ActiveUnitAction>,
    range_highlights: Option<Res<RangeHighlights>>,
    path_highlights: Option<Res<PathHighlights>>,
    mouse_cursor_on_tile: Option<Res<CursorOnTile>>,
    mut event_writer: EventWriter<ClientToServerMessage>,
    mut next_combat_state: ResMut<NextState<CombatState>>,
    game_data: Res<GameData>,
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
                error!("Range Highlights didn't exist?");
                return;
            };

            if !range_highlights.tiles.contains(selected_tile) {
                return;
            }

            let Some(path_highlights) = path_highlights else {
                error!("Path Highlights didn't exist?");
                return;
            };

            event_writer.send(ClientToServerMessage::MoveUnit(
                client_to_server::MoveUnit {
                    path: path_highlights.tiles.clone(),
                },
            ));
        }
        ActiveUnitAction::UseSkill(id) => {
            let skill = &game_data.skills[id];
            if skill.targeting != SkillTargeting::UserPosition {
                let Some(range_highlights) = range_highlights else {
                    error!("Range Highlights didn't exist?");
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
    mut combat_data: ResMut<CombatData>,
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

        if unit.owner == local_player_id.owner {
            next_combat_state.set(CombatState::ThisPlayerUnitTurn);
        } else {
            next_combat_state.set(CombatState::WaitingForOtherPlayer);
        }
    }
}

pub fn on_use_skill(
    mut commands: Commands,
    map: Res<GameMap>,
    locals: Res<LocalCombatData>,
    mut events: EventReader<server_to_client::UseSkill>,
    mut combat_data: ResMut<CombatData>,
    mut next_combat_state: ResMut<NextState<CombatState>>,
    local_player_id: Res<LocalPlayerId>,
    character_sprites: Res<CharacterSprites>,
    mut sprite_params: Sprite3dParams,
) {
    for event in events.read() {
        for x in &event.hits {
            let target = combat_data.units.get_mut(&x.target_unit_id).unwrap();

            // TODO: Stat reduction should be extracted into common
            // TODO: Consider extracting this into a "unit takes damage"-Event
            if target.hp < x.physical_damage {
                target.hp = 0;

                let entity = locals.unit_entities[&target.id];
                commands
                    .entity(entity)
                    .insert(sprite_builder::build_dead_unit_sprite(
                        target,
                        &character_sprites,
                        &map,
                        &mut sprite_params,
                    ));
            } else {
                target.hp -= x.physical_damage;
            }
        }

        let turn = combat_data.current_turn.as_unit_turn_mut().unwrap();
        turn.remaining_actions -= 1;
        let unit_id = turn.unit_id;

        let unit = combat_data.units.get(&unit_id).unwrap();

        commands
            .entity(locals.unit_entities[&unit_id])
            .insert(UnitAttackAnimationComponent::new(
                unit,
                event.target_coordinates,
                &map,
            ));

        if unit.owner == local_player_id.owner {
            next_combat_state.set(CombatState::ThisPlayerUnitTurn);
        } else {
            next_combat_state.set(CombatState::WaitingForOtherPlayer);
        }
    }
}
