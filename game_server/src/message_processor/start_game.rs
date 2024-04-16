use crate::message_processor::ServerToClientMessageVariant;
use crate::state::ServerState::InGame;
use crate::state::{ServerData, SharedState};
use game_common::combat_data::CombatData;
use game_common::game_map::GameMap;
use game_common::network_events::server_to_client::{
    ErrorWhenProcessingMessage, ServerToClientMessage, StartGameAndLoadMap,
};
use game_common::TEST_MAP_NAME;

pub fn start_game(
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
    let combat_state = CombatData {
        units: Default::default(),
        unit_positions: Default::default(),
        turn_order: Default::default(),
        unit_storage: Default::default(),
        current_unit_turn: None,
        turn_resources: Default::default(),
    };
    let server_data = ServerData {
        combat_data: combat_state,
        loaded_map: map,
    };

    shared_state.server_state = InGame(server_data);

    Ok(vec![ServerToClientMessageVariant::Broadcast(
        ServerToClientMessage::LoadMap(StartGameAndLoadMap {
            path: TEST_MAP_NAME.into(),
        }),
    )])
}