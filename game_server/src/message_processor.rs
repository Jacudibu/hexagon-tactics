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
        ClientToServerMessage::StartGame => {
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
            shared_state.game_states.push(game_state);

            Ok(ServerToClientMessage::LoadMap(LoadMap {
                path: TEST_MAP_NAME.into(),
            }))
        }
    }
}
