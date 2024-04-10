mod combat_input;
mod combat_ui;

use crate::combat::combat_input::CombatInputPlugin;
use crate::combat::combat_ui::CombatUiPlugin;
use crate::map::MapState;
use crate::ApplicationState;
use bevy::app::App;
use bevy::prelude::{
    in_state, info, on_event, Commands, EventReader, EventWriter, IntoSystemConfigs, NextState,
    OnEnter, Plugin, Reflect, Res, ResMut, Resource, States, Update,
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
        app.init_state::<CombatState>();
        app.add_systems(
            OnEnter(MapState::Loaded),
            on_map_loaded.run_if(in_state(ApplicationState::InGame)),
        );
        app.add_systems(
            Update,
            (
                on_add_unit_to_player.run_if(on_event::<AddUnitToPlayer>()),
                on_player_turn_to_place_unit
                    .run_if(on_event::<PlayerTurnToPlaceUnit>())
                    // TODO: Later on this chain isn't needed anymore since the client will already know which units it owns way ahead, but right now it's very likely to happen in the same frame
                    .after(on_add_unit_to_player),
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
    // TODO: Maybe have a separate struct for CurrentlyPlacedUnit?
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
    mut commands: Commands,
    mut event: EventReader<PlayerTurnToPlaceUnit>,
    combat_data: Res<CombatData>,
    mut next_combat_state: ResMut<NextState<CombatState>>,
) {
    for event in event.read() {
        if event.player == CONSTANT_LOCAL_PLAYER_ID {
            next_combat_state.set(CombatState::PlaceUnit);
            let unit_id = combat_data.units_that_can_still_be_placed.first().unwrap();
            commands.insert_resource(CurrentlySelectedUnit {
                unit_id: unit_id.clone(),
            });
        } else {
            next_combat_state.set(CombatState::WaitingForOtherPlayer)
        }
    }
}
