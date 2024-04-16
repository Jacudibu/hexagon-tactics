use crate::combat::combat_input::CombatInputPlugin;
use crate::combat::combat_ui::CombatUiPlugin;
use crate::combat::end_turn::EndTurnPlugin;
use crate::combat::local_combat_data::LocalCombatData;
use crate::combat::unit_actions::UnitActionPlugin;
use crate::combat::unit_placement::UnitPlacementPlugin;
use crate::map::MapState;
use crate::ApplicationState;
use bevy::app::{App, Plugin, Update};
use bevy::log::info;
use bevy::prelude::{
    error, in_state, on_event, Commands, EventReader, EventWriter, IntoSystemConfigs, NextState,
    OnEnter, Reflect, ResMut, States,
};
use game_common::combat_data::CombatData;
use game_common::combat_turn::CombatTurn;
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::server_to_client::{
    AddUnitToPlayerStorage, PlayerTurnToPlaceUnit, StartUnitTurn,
};
use game_common::network_events::CONSTANT_LOCAL_PLAYER_ID;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CombatUiPlugin);
        app.add_plugins(CombatInputPlugin);
        app.add_plugins(UnitPlacementPlugin);
        app.add_plugins(UnitActionPlugin);
        app.add_plugins(EndTurnPlugin);
        app.init_state::<CombatState>();
        app.add_systems(
            OnEnter(MapState::Loaded),
            on_map_loaded.run_if(in_state(ApplicationState::InGame)),
        );
        app.add_systems(
            Update,
            (
                on_add_unit_to_player_storage.run_if(on_event::<AddUnitToPlayerStorage>()),
                on_player_turn_to_place_unit.run_if(on_event::<PlayerTurnToPlaceUnit>()),
                on_start_unit_turn.run_if(on_event::<StartUnitTurn>()),
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
    ThisPlayerUnitTurn,
    OtherPlayerUnitTurn, // TODO: Specify when this should be used rather than WaitingForOtherPlayer
}

pub fn on_map_loaded(
    mut commands: Commands,
    mut client_to_server_messages: EventWriter<ClientToServerMessage>,
) {
    client_to_server_messages.send(ClientToServerMessage::FinishedLoading);
    commands.insert_resource(CombatData {
        units: Default::default(),
        unit_positions: Default::default(),
        unit_storage: vec![],
        current_turn: CombatTurn::Undefined,
    });
    commands.insert_resource(LocalCombatData {
        unit_entities: Default::default(),
    });
}

pub fn on_add_unit_to_player_storage(
    mut add_unit_to_player_event: EventReader<AddUnitToPlayerStorage>,
    mut combat_data: ResMut<CombatData>,
) {
    for x in add_unit_to_player_event.read() {
        combat_data.unit_storage.push(x.unit.clone());
        info!("Received unit: {:?}", x)
    }
}

pub fn on_player_turn_to_place_unit(
    mut event: EventReader<PlayerTurnToPlaceUnit>,
    mut next_combat_state: ResMut<NextState<CombatState>>,
    mut combat_data: ResMut<CombatData>,
) {
    for event in event.read() {
        if event.player == CONSTANT_LOCAL_PLAYER_ID {
            next_combat_state.set(CombatState::PlaceUnit);
            combat_data.current_turn = CombatTurn::place_unit(event.player);
        } else {
            next_combat_state.set(CombatState::WaitingForOtherPlayer);
            combat_data.current_turn = CombatTurn::place_unit(event.player);
        }
    }
}

pub fn on_start_unit_turn(
    mut event: EventReader<StartUnitTurn>,
    mut next_combat_state: ResMut<NextState<CombatState>>,
    mut combat_data: ResMut<CombatData>,
) {
    for event in event.read() {
        let locally_computed_next = combat_data.get_next_unit();
        if locally_computed_next != event.unit_id {
            error!("Turn order seems to be out of sync? Server said to start turn for unit {}, but locally, it would be {}'s turn.",
                event.unit_id, locally_computed_next
            );
        }

        let Some(unit) = combat_data.units.get(&event.unit_id) else {
            error!("Was unable to find unit with ID {}.", event.unit_id);
            continue;
        };

        if unit.owner == CONSTANT_LOCAL_PLAYER_ID {
            next_combat_state.set(CombatState::ThisPlayerUnitTurn);
        } else {
            next_combat_state.set(CombatState::OtherPlayerUnitTurn)
        }

        combat_data.start_unit_turn(event.unit_id);
    }
}
