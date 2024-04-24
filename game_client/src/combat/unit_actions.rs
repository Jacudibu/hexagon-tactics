use crate::combat::combat_input::CombatAction;
use crate::combat::combat_plugin::CombatState;
use crate::combat::local_combat_data::LocalCombatData;
use crate::combat::unit_placement;
use crate::combat::unit_placement::UnitMarker;
use crate::map::{HighlightedTiles, MouseCursorOnTile};
use crate::networking::LocalPlayerId;
use crate::ApplicationState;
use bevy::app::App;
use bevy::prelude::{
    error, in_state, on_event, resource_changed_or_removed, resource_exists, Commands, Event,
    EventReader, EventWriter, IntoSystemConfigs, NextState, Plugin, PreUpdate, Query, Res, ResMut,
    Resource, Transform, Update, With,
};
use game_common::combat_data::CombatData;
use game_common::game_map::GameMap;
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::{client_to_server, server_to_client};
use game_common::skill::{Skill, SkillId};
use game_common::DESYNC_TODO_MESSAGE;
use hexx::Hex;
use leafwing_input_manager::action_state::ActionState;
use std::cmp::PartialEq;
use std::ops::Deref;

pub struct UnitActionPlugin;
impl Plugin for UnitActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SetOrToggleActiveUnitActionCommand>();
        app.add_systems(
            PreUpdate,
            (
                change_action_on_input,
                set_or_toggle_action.run_if(on_event::<SetOrToggleActiveUnitActionCommand>()),
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
                    .run_if(resource_exists::<ActiveUnitAction>)
                    .run_if(resource_exists::<HighlightedTiles>),
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
pub struct SetOrToggleActiveUnitActionCommand {
    pub action: ActiveUnitAction,
}

pub fn set_or_toggle_action(
    mut commands: Commands,
    current_action: Option<Res<ActiveUnitAction>>,
    mut events: EventReader<SetOrToggleActiveUnitActionCommand>,
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
    mut event_writer: EventWriter<SetOrToggleActiveUnitActionCommand>,
) {
    if action_state.just_pressed(&CombatAction::MoveUnit) {
        event_writer.send(SetOrToggleActiveUnitActionCommand {
            action: ActiveUnitAction::Move,
        });
    }
}

pub fn on_active_unit_action_changed(
    mut commands: Commands,
    combat_data: Res<CombatData>,
    map: Res<GameMap>,
    active_unit_action: Option<Res<ActiveUnitAction>>,
) {
    let Some(active_unit_action) = active_unit_action else {
        commands.remove_resource::<HighlightedTiles>();
        return;
    };

    match active_unit_action.deref() {
        ActiveUnitAction::Move => {
            show_movement_range_preview(commands, &combat_data, &map);
        }
        ActiveUnitAction::UseSkill(id) => {
            show_skill_range_preview(id, commands, &combat_data, &map);
        }
    }
}

pub fn show_movement_range_preview(
    mut commands: Commands,
    combat_data: &CombatData,
    map: &GameMap,
) {
    let unit = combat_data
        .units
        .get(&combat_data.current_turn.as_unit_turn().unwrap().unit_id)
        .expect("TODO");

    let range = map.field_of_movement(unit, combat_data);

    commands.insert_resource(HighlightedTiles { tiles: range })
}

pub fn show_skill_range_preview(
    _skill_id: &SkillId,
    mut commands: Commands,
    combat_data: &CombatData,
    map: &GameMap,
) {
    let unit = combat_data
        .units
        .get(&combat_data.current_turn.as_unit_turn().unwrap().unit_id)
        .expect("TODO");

    let skill = Skill::debug_attack();

    let tiles: Vec<Hex> = hexx::algorithms::range_fov(unit.position, skill.range.max, |hex| {
        !map.tiles.contains_key(&hex)
    })
    .into_iter()
    .filter(|x| x.unsigned_distance_to(unit.position) >= skill.range.min)
    .collect();

    commands.insert_resource(HighlightedTiles { tiles })
}

pub fn execute_action_on_click(
    mut commands: Commands,
    combat_data: Res<CombatData>,
    map: Res<GameMap>,
    action_state: Res<ActionState<CombatAction>>,
    active_unit_action: Res<ActiveUnitAction>,
    highlighted_tiles: Res<HighlightedTiles>,
    mouse_cursor_on_tile: Option<Res<MouseCursorOnTile>>,
    mut event_writer: EventWriter<ClientToServerMessage>,
    mut next_combat_state: ResMut<NextState<CombatState>>,
) {
    if !action_state.just_pressed(&CombatAction::SelectTile) {
        return;
    }

    let Some(mouse_cursor_on_tile) = mouse_cursor_on_tile else {
        return;
    };

    let selected_tile = &mouse_cursor_on_tile.hex;
    if !highlighted_tiles.tiles.contains(selected_tile) {
        return;
    }

    match active_unit_action.deref() {
        ActiveUnitAction::Move => {
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
    mut events: EventReader<server_to_client::MoveUnit>,
    mut combat_data: ResMut<CombatData>,
    local_combat_data: Res<LocalCombatData>,
    map: Res<GameMap>,
    mut unit_entities: Query<&mut Transform, With<UnitMarker>>,
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

        // TODO: Animate Movement
        if let Ok(mut transform) = unit_entities.get_mut(entity) {
            transform.translation = unit_placement::unit_position_on_hexagon(unit.position, &map)
        }

        if unit.owner == local_player_id.id {
            next_combat_state.set(CombatState::ThisPlayerUnitTurn);
        } else {
            next_combat_state.set(CombatState::WaitingForOtherPlayer);
        }
    }
}

pub fn on_use_skill(
    mut events: EventReader<server_to_client::UseSkill>,
    mut combat_data: ResMut<CombatData>,
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

#[cfg(test)]
mod tests {
    use crate::combat::unit_actions::{ActiveUnitAction, UnitActionPlugin};
    use crate::map::HighlightedTiles;
    use crate::networking::NetworkPlugin;
    use bevy::app::App;
    use game_common::combat_data::CombatData;
    use game_common::game_map::GameMap;
    use game_common::unit::Unit;
    use hexx::Hex;

    #[test]
    fn should_create_and_remove_highlights() {
        let mut app = App::new();
        app.add_plugins(UnitActionPlugin);
        app.add_plugins(NetworkPlugin);
        app.insert_resource(GameMap::new(1));

        let unit_id = 1;
        let unit = Unit::create_mock(unit_id, 1).with_position(Hex::ZERO);

        app.insert_resource(
            CombatData::create_mock()
                .with_units(vec![unit])
                .with_unit_turn(unit_id),
        );
        app.insert_resource(ActiveUnitAction::Move);
        app.update();

        assert!(
            app.world.get_resource::<HighlightedTiles>().is_some(),
            "HighlightedTiles should have been created!"
        );

        app.world.remove_resource::<ActiveUnitAction>();
        app.update();

        assert!(
            app.world.get_resource::<HighlightedTiles>().is_none(),
            "HighlightedTiles should have been removed!"
        );
    }
}
