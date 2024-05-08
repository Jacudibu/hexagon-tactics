use crate::in_game_state::{InGameData, InGameState, MatchData};
use crate::message_processor::command_invocation_result::StateTransition;
use crate::message_processor::ServerToClientMessageVariant;
use game_common::combat_data::CombatData;
use game_common::combat_turn::CombatTurn;
use game_common::game_map::GameMap;
use game_common::network_events::server_to_client::{
    ErrorWhenProcessingMessage, ServerToClientMessage, StartGameAndLoadMap,
};
use game_common::player::PlayerId;
use game_common::TEST_MAP_NAME;

#[must_use]
pub fn handle_transition(
    sender: &PlayerId,
    state_transition: &StateTransition,
    in_game_data: &mut InGameData,
) -> Vec<ServerToClientMessageVariant> {
    let players_in_state = in_game_data.get_all_players_in_same_state(sender);

    match state_transition {
        StateTransition::StartCombat => start_combat(in_game_data, players_in_state),
    }
}

pub fn start_combat(
    in_game_data: &mut InGameData,
    players_in_state: Vec<PlayerId>,
) -> Vec<ServerToClientMessageVariant> {
    let mut result = Vec::new();
    let map = match GameMap::load_from_file(TEST_MAP_NAME) {
        Ok(map) => map,
        Err(e) => {
            // TODO: Load a "known good" map instead, and send player the correct map id
            result.push(ServerToClientMessageVariant::SendToSender(
                ServerToClientMessage::ErrorWhenProcessingMessage(ErrorWhenProcessingMessage {
                    message: "Server failed to load map!".into(),
                }),
            ));
            GameMap::new(2)
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

    in_game_data.insert_state_for_player(players_in_state[0], InGameState::Combat(match_data));
    for player_id in 1..players_in_state.len() {
        in_game_data.add_player_to_other_player_state(&players_in_state[0], player_id.clone());
    }

    result.push(ServerToClientMessageVariant::Broadcast(
        ServerToClientMessage::LoadMap(StartGameAndLoadMap {
            path: TEST_MAP_NAME.into(),
        }),
    ));

    result
}
