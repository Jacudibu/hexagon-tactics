use crate::combat::combat_input::CombatAction;
use crate::combat::CombatState;
use crate::map::MouseCursorOnTile;
use bevy::prelude::*;
use game_common::game_state::CombatData;
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::{client_to_server, server_to_client};
use game_common::units::UnitId;
use leafwing_input_manager::action_state::ActionState;

pub struct UnitPlacementPlugin;
impl Plugin for UnitPlacementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CombatState::PlaceUnit), setup_state);
        app.add_systems(OnExit(CombatState::PlaceUnit), leave_state);
        app.add_systems(
            Update,
            (
                on_server_placed_unit.run_if(on_event::<server_to_client::PlaceUnit>()),
                input_listener.run_if(in_state(CombatState::PlaceUnit)),
            ),
        );
    }
}

#[derive(Resource, Debug)]
pub struct CurrentlyPlacedUnit {
    pub unit_id: UnitId, // Consider just storing the array_index
    array_index: usize,
}

fn setup_state(mut commands: Commands, combat_data: Res<CombatData>) {
    let unit_id = combat_data.units_that_can_still_be_placed.first().unwrap();
    commands.insert_resource(CurrentlyPlacedUnit {
        unit_id: unit_id.clone(),
        array_index: 0,
    });
}

fn leave_state(mut commands: Commands) {
    commands.remove_resource::<CurrentlyPlacedUnit>();
}

fn input_listener(
    mut currently_placed_unit: ResMut<CurrentlyPlacedUnit>,
    action_state: Res<ActionState<CombatAction>>,
    combat_data: Res<CombatData>,
    cursor: Option<Res<MouseCursorOnTile>>,
    mut client_to_server_events: EventWriter<ClientToServerMessage>,
) {
    let units = &combat_data.units_that_can_still_be_placed;
    if action_state.just_pressed(&CombatAction::NextUnit) {
        if currently_placed_unit.array_index + 1 >= units.len() {
            currently_placed_unit.array_index = 0;
        } else {
            currently_placed_unit.array_index += 1;
        }

        currently_placed_unit.unit_id = units[currently_placed_unit.array_index].clone();
    } else if action_state.just_pressed(&CombatAction::PreviousUnit) {
        if currently_placed_unit.array_index == 0 {
            currently_placed_unit.array_index = units.len() - 1;
        } else {
            currently_placed_unit.array_index -= 1;
        }

        currently_placed_unit.unit_id = units[currently_placed_unit.array_index].clone();
    } else if action_state.just_pressed(&CombatAction::SelectTile) {
        if let Some(cursor) = cursor {
            // TODO: Validation
            client_to_server_events.send(ClientToServerMessage::PlaceUnit(
                client_to_server::PlaceUnit {
                    unit_id: currently_placed_unit.unit_id,
                    hex: cursor.hex,
                },
            ));
        }
    }
}

fn on_server_placed_unit(
    mut events: EventReader<server_to_client::PlaceUnit>,
    mut combat_data: ResMut<CombatData>,
    mut next_state: ResMut<NextState<CombatState>>,
) {
    for event in events.read() {
        combat_data
            .units_that_can_still_be_placed
            .retain(|x| x != &event.unit_id);
        combat_data.unit_positions.insert(event.hex, event.unit_id);

        info!(
            "PlaceUnit received for {} on {:?}",
            event.unit_id, event.hex
        );

        next_state.set(CombatState::WaitingForServer)
    }
}
