use crate::map::MapState;
use crate::ApplicationState;
use bevy::app::App;
use bevy::prelude::{
    in_state, info, on_event, Commands, EventReader, EventWriter, IntoSystemConfigs, OnEnter,
    Plugin, Reflect, Res, ResMut, States, Update,
};
use game_common::game_map::GameMap;
use game_common::game_state::GameState;
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::server_to_client::AddUnitToPlayer;

pub struct CombatPlugin;
impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(MapState::Loaded),
            on_finish_loading.run_if(in_state(ApplicationState::InGame)),
        );
        app.add_systems(
            Update,
            on_add_unit_to_player
                .run_if(on_event::<AddUnitToPlayer>())
                .run_if(in_state(ApplicationState::InGame)),
        );
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Reflect)]
pub enum CombatState {
    #[default]
    WaitingForServer,
    LocalPlayerTurn,
}

pub fn on_finish_loading(
    mut commands: Commands,
    map: Res<GameMap>,
    mut network_event: EventWriter<ClientToServerMessage>,
) {
    let map = map.clone(); // TODO: No. Nope. Nada. Extract GameMap from GameState.

    commands.insert_resource(GameState {
        map,
        units: Default::default(),
        unit_positions: Default::default(),
        turn_order: Default::default(),
        units_that_can_still_be_placed: Default::default(),
    });

    network_event.send(ClientToServerMessage::FinishedLoading);
}

pub fn on_add_unit_to_player(
    mut add_unit_to_player_event: EventReader<AddUnitToPlayer>,
    mut game_state: ResMut<GameState>,
) {
    for x in add_unit_to_player_event.read() {
        game_state
            .units_that_can_still_be_placed
            .push(x.unit.clone());
        info!("Received unit: {:?}", x)
    }
}
