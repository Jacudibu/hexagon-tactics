use crate::SharedState;
use game_common::game_map::GameMap;
use game_common::game_state::GameState;
use game_common::network_message::{DebugMessage, LoadMap, NetworkMessage};
use game_common::TEST_MAP_NAME;

pub fn process_message(
    shared_state: &mut SharedState,
    message: NetworkMessage,
) -> Result<NetworkMessage, ()> {
    match message {
        NetworkMessage::StartGame => {
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

            Ok(NetworkMessage::LoadMap(LoadMap {
                path: TEST_MAP_NAME.into(),
            }))
        }
        // NetworkMessage::LoadMap(load_map) => {}
        // NetworkMessage::MoveUnit(_) => {}
        // NetworkMessage::DebugMessage(_) => {}
        _ => Ok(NetworkMessage::DebugMessage(DebugMessage {
            message: format!("received {:?}", message),
        })),
    }
}
