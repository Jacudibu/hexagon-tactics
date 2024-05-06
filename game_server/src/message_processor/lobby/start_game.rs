use crate::in_game_state::{InGameData, InGameState, MatchData};
use crate::message_processor::ServerToClientMessageVariant;
use crate::shared_state::ServerState::InGame;
use crate::shared_state::SharedState;
use game_common::combat_data::CombatData;
use game_common::combat_turn::CombatTurn;
use game_common::game_map::GameMap;
use game_common::network_events::server_to_client::{
    ErrorWhenProcessingMessage, ServerToClientMessage, StartGameAndLoadMap,
};
use game_common::player::ReadyState;
use game_common::{validation, TEST_MAP_NAME};

pub fn start_game(
    shared_state: &mut SharedState,
) -> Result<Vec<ServerToClientMessageVariant>, ServerToClientMessage> {
    validation::validate_player_readiness(&shared_state.players, &ReadyState::ReadyInLobby)?;

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
    let combat_data = CombatData {
        units: Default::default(),
        unit_positions: Default::default(),
        unit_storage: Default::default(),
        current_turn: CombatTurn::Undefined,
    };
    let match_data = MatchData {
        combat_data,
        loaded_map: map,
    };

    let mut in_game_data = InGameData::default();
    let first = shared_state.players.keys().find(|_| true).unwrap();
    in_game_data.insert_state_for_player(first.clone(), InGameState::Combat(match_data));

    for player_id in shared_state.players.keys().filter(|&x| x != first) {
        in_game_data.add_player_to_other_player_state(first, player_id.clone());
    }

    shared_state.server_state = InGame(in_game_data);

    Ok(vec![ServerToClientMessageVariant::Broadcast(
        ServerToClientMessage::LoadMap(StartGameAndLoadMap {
            path: TEST_MAP_NAME.into(),
        }),
    )])
}
