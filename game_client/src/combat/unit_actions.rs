use crate::combat::combat_input::CombatAction;
use crate::combat::combat_plugin::CombatState;
use crate::combat::local_combat_data::LocalCombatData;
use crate::combat::unit_placement;
use crate::combat::unit_placement::UnitMarker;
use crate::map::{HighlightedTiles, MouseCursorOnTile};
use crate::ApplicationState;
use bevy::app::App;
use bevy::prelude::{
    error, in_state, info, on_event, resource_changed_or_removed, resource_exists, Commands,
    EventReader, EventWriter, IntoSystemConfigs, NextState, Plugin, Query, Res, ResMut, Resource,
    Transform, Update, With,
};
use game_common::combat_data::CombatData;
use game_common::game_map::GameMap;
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::{client_to_server, server_to_client};
use game_common::DESYNC_TODO_MESSAGE;
use leafwing_input_manager::action_state::ActionState;
use std::ops::Deref;

pub struct UnitActionPlugin;
impl Plugin for UnitActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                change_action_on_input
                    .run_if(in_state(ApplicationState::InGame))
                    .run_if(in_state(CombatState::ThisPlayerUnitTurn)),
                on_active_unit_action_changed
                    .run_if(resource_changed_or_removed::<ActiveUnitAction>()),
                execute_action_on_click
                    .run_if(in_state(ApplicationState::InGame))
                    .run_if(in_state(CombatState::ThisPlayerUnitTurn))
                    .run_if(resource_exists::<ActiveUnitAction>)
                    .run_if(resource_exists::<HighlightedTiles>),
                on_move_unit.run_if(on_event::<server_to_client::MoveUnit>()),
            ),
        );
    }
}

#[derive(Resource, Eq, PartialEq)]
pub enum ActiveUnitAction {
    Move,
}

pub fn change_action_on_input(
    mut commands: Commands,
    action_state: Res<ActionState<CombatAction>>,
    current_action: Option<Res<ActiveUnitAction>>,
) {
    if action_state.just_pressed(&CombatAction::MoveUnit) {
        if let Some(current_action) = current_action {
            if current_action.deref() == &ActiveUnitAction::Move {
                commands.remove_resource::<ActiveUnitAction>();
                return;
            }
        }

        commands.insert_resource(ActiveUnitAction::Move)
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
    }
}

pub fn show_movement_range_preview(
    mut commands: Commands,
    combat_data: &CombatData,
    map: &GameMap,
) {
    let unit = combat_data
        .units
        .get(&combat_data.current_unit_turn.expect("TODO"))
        .expect("TODO");

    let range = map.field_of_movement(unit, combat_data);

    commands.insert_resource(HighlightedTiles { tiles: range })
}

pub fn execute_action_on_click(
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
        // TODO: Path should already exist somewhere for highlighting/preview
        ActiveUnitAction::Move => {
            let Some(path) = map.calculate_path(&combat_data, selected_tile.clone()) else {
                error!(
                    "Unable to calculate unit path for unit_id {:?} to {:?}",
                    combat_data.current_unit_turn, selected_tile
                );
                return;
            };
            event_writer.send(ClientToServerMessage::MoveUnit(
                client_to_server::MoveUnit { path },
            ));
            next_combat_state.set(CombatState::WaitingForServer);
        }
    }
}

pub fn on_move_unit(
    mut events: EventReader<server_to_client::MoveUnit>,
    mut combat_data: ResMut<CombatData>,
    local_combat_data: Res<LocalCombatData>,
    map: Res<GameMap>,
    mut unit_entities: Query<&mut Transform, With<UnitMarker>>,
) {
    for event in events.read() {
        let unit_id = combat_data.current_unit_turn.expect(DESYNC_TODO_MESSAGE);
        let unit = combat_data
            .units
            .get_mut(&unit_id)
            .expect(DESYNC_TODO_MESSAGE);

        unit.position = Some(
            event
                .path
                .last()
                .expect("Path sent from server should never be empty!")
                .clone(),
        );

        let entity = local_combat_data.unit_entities[&unit_id];
        if let Ok(mut transform) = unit_entities.get_mut(entity) {
            transform.translation =
                unit_placement::unit_position_on_hexagon(unit.position.unwrap(), &map)
        }

        // TODO: reduce unit movement
    }
}

#[cfg(test)]
mod tests {
    use crate::combat::unit_actions::{ActiveUnitAction, UnitActionPlugin};
    use crate::map::HighlightedTiles;
    use bevy::app::App;
    use game_common::combat_data::CombatData;
    use game_common::game_map::GameMap;
    use game_common::unit::Unit;
    use hexx::Hex;

    #[test]
    fn should_create_and_remove_highlights() {
        let mut app = App::new();
        app.add_plugins(UnitActionPlugin);
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
