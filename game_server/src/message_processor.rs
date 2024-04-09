use crate::state::ServerState::InGame;
use crate::state::{ServerState, SharedState};
use game_common::game_map::GameMap;
use game_common::game_state::GameState;
use game_common::network_events::client_to_server::*;
use game_common::network_events::server_to_client::*;
use game_common::units::Unit;
use game_common::TEST_MAP_NAME;
use tracing::error;

#[derive(Debug)]
pub enum ServerToClientMessageVariant {
    SendToSender(ServerToClientMessage),
    SendToEveryoneExceptSender(ServerToClientMessage),
    Broadcast(ServerToClientMessage),
}

pub fn process_message(
    shared_state: &mut SharedState,
    message: ClientToServerMessage,
) -> Result<Vec<ServerToClientMessageVariant>, ServerToClientMessage> {
    match message {
        ClientToServerMessage::StartGame => start_game(shared_state),
        ClientToServerMessage::FinishedLoading => finish_loading(shared_state),
    }
}

fn start_game(
    shared_state: &mut SharedState,
) -> Result<Vec<ServerToClientMessageVariant>, ServerToClientMessage> {
    let map = match GameMap::load_from_file(TEST_MAP_NAME) {
        Ok(map) => map,
        Err(_) => {
            return Err(ServerToClientMessage::ErrorWhenProcessingMessage(
                ErrorWhenProcessingMessage {
                    message: "Server failed to load map!".into(),
                },
            ))
        }
    };
    let game_state = GameState {
        map,
        units: Default::default(),
        unit_positions: Default::default(),
        turn_order: Default::default(),
        units_that_can_still_be_placed: Default::default(),
    };
    shared_state.server_state = InGame(game_state);

    Ok(vec![ServerToClientMessageVariant::Broadcast(
        ServerToClientMessage::LoadMap(StartGameAndLoadMap {
            path: TEST_MAP_NAME.into(),
        }),
    )])
}

fn finish_loading(
    shared_state: &mut SharedState,
) -> Result<Vec<ServerToClientMessageVariant>, ServerToClientMessage> {
    // TODO: Acquire Actual PlayerId
    let player_id = 1;

    let unit_a = Unit::create_debug_unit(1, player_id, "Unit A".into());
    let unit_b = Unit::create_debug_unit(2, player_id, "Unit B".into());
    let unit_c = Unit::create_debug_unit(3, player_id, "Unit C".into());

    match shared_state.server_state {
        ServerState::WaitingForConnection => {
            error!("Wrong server state to receive FinishLoading events!")
        }
        InGame(ref mut game_state) => {
            game_state
                .units_that_can_still_be_placed
                .push(unit_a.clone());
            game_state
                .units_that_can_still_be_placed
                .push(unit_b.clone());
            game_state
                .units_that_can_still_be_placed
                .push(unit_c.clone());
        }
    }

    // TODO: Check if all players are ready
    Ok(vec![
        ServerToClientMessageVariant::Broadcast(ServerToClientMessage::PlayerIsReady(
            PlayerIsReady { player_id },
        )),
        ServerToClientMessageVariant::Broadcast(ServerToClientMessage::AddUnitToPlayer(
            AddUnitToPlayer { unit: unit_a },
        )),
        ServerToClientMessageVariant::Broadcast(ServerToClientMessage::AddUnitToPlayer(
            AddUnitToPlayer { unit: unit_b },
        )),
        ServerToClientMessageVariant::Broadcast(ServerToClientMessage::AddUnitToPlayer(
            AddUnitToPlayer { unit: unit_c },
        )),
    ])
}
