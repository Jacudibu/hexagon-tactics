use crate::ServerState::InGame;
use crate::SharedState;
use game_common::game_map::GameMap;
use game_common::game_state::GameState;
use game_common::network_events::client_to_server::*;
use game_common::network_events::server_to_client::*;
use game_common::TEST_MAP_NAME;

pub fn process_message(
    shared_state: &mut SharedState,
    message: ClientToServerMessage,
) -> Result<ServerToClientMessage, ()> {
    match message {
        ClientToServerMessage::StartGame => start_game(shared_state),
        ClientToServerMessage::FinishedLoading => finish_loading(shared_state),
    }
}

fn start_game(shared_state: &mut SharedState) -> Result<ServerToClientMessage, ()> {
    let map = match GameMap::load_from_file(TEST_MAP_NAME) {
        Ok(map) => map,
        Err(_) => return Err(()),
    };
    let game_state = GameState {
        map,
        units: Default::default(),
        unit_positions: Default::default(),
        turn_order: Default::default(),
    };
    shared_state.server_state = InGame(game_state);

    Ok(ServerToClientMessage::LoadMap(StartGameAndLoadMap {
        path: TEST_MAP_NAME.into(),
    }))
}

fn finish_loading(shared_state: &mut SharedState) -> Result<ServerToClientMessage, ()> {
    // TODO: Check if all players are ready
    // TODO: Acquire Actual PlayerId
    let player_id = 1;

    // TODO: Add a way to send multiple events in case all players are ready
    Ok(ServerToClientMessage::PlayerIsReady(PlayerIsReady {
        player_id,
    }))
}
