use crate::message_processor::ServerToClientMessageVariant;
use crate::state::ServerState::InGame;
use crate::state::{MatchData, SharedState};
use game_common::combat_data::CombatData;
use game_common::combat_turn::CombatTurn;
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
        unit_storage: Default::default(),
        current_turn: CombatTurn::Undefined,
    };
    let server_data = MatchData {
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
