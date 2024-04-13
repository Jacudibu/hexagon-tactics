use crate::combat::combat_input::CombatInputPlugin;
use crate::combat::combat_ui::CombatUiPlugin;
use crate::combat::unit_placement::UnitPlacementPlugin;
use crate::map::MapState;
use crate::ApplicationState;
use bevy::app::{App, Plugin, Update};
use bevy::log::info;
use bevy::prelude::{
    in_state, on_event, Commands, EventReader, EventWriter, IntoSystemConfigs, NextState, OnEnter,
    Reflect, ResMut, Resource, States,
};
use game_common::game_state::CombatData;
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::server_to_client::{AddUnitToPlayer, PlayerTurnToPlaceUnit};
use game_common::network_events::CONSTANT_LOCAL_PLAYER_ID;
use game_common::units::UnitId;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CombatUiPlugin);
        app.add_plugins(CombatInputPlugin);
        app.add_plugins(UnitPlacementPlugin);
        app.init_state::<CombatState>();
        app.add_systems(
            OnEnter(MapState::Loaded),
            on_map_loaded.run_if(in_state(ApplicationState::InGame)),
        );
        app.add_systems(
            Update,
            (
                on_add_unit_to_player.run_if(on_event::<AddUnitToPlayer>()),
                on_player_turn_to_place_unit.run_if(on_event::<PlayerTurnToPlaceUnit>()),
            )
                .run_if(in_state(ApplicationState::InGame)),
        );
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Reflect)]
pub enum CombatState {
    #[default]
    WaitingForServer,
    WaitingForOtherPlayer,
    PlaceUnit,
}

#[derive(Resource, Debug)]
pub struct CurrentlySelectedUnit {
    unit_id: UnitId,
}

pub fn on_map_loaded(
    mut commands: Commands,
    mut client_to_server_messages: EventWriter<ClientToServerMessage>,
) {
    client_to_server_messages.send(ClientToServerMessage::FinishedLoading);
    commands.insert_resource(CombatData {
        units: Default::default(),
        unit_positions: Default::default(),
        turn_order: Default::default(),
        units_that_can_still_be_placed: vec![],
    })
}

pub fn on_add_unit_to_player(
    mut add_unit_to_player_event: EventReader<AddUnitToPlayer>,
    mut combat_data: ResMut<CombatData>,
) {
    for x in add_unit_to_player_event.read() {
        combat_data.units_that_can_still_be_placed.push(x.unit.id);
        combat_data.units.insert(x.unit.id, x.unit.clone());
        info!("Received unit: {:?}", x)
    }
}

pub fn on_player_turn_to_place_unit(
    mut event: EventReader<PlayerTurnToPlaceUnit>,
    mut next_combat_state: ResMut<NextState<CombatState>>,
) {
    for event in event.read() {
        if event.player == CONSTANT_LOCAL_PLAYER_ID {
            next_combat_state.set(CombatState::PlaceUnit);
        } else {
            next_combat_state.set(CombatState::WaitingForOtherPlayer)
        }
    }
}
