use crate::in_game_state::MatchData;
use crate::message_processor::{create_error_response, ServerToClientMessageVariant};
use game_common::game_data::GameData;
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::server_to_client::ServerToClientMessage;
use game_common::player::{Player, PlayerId};
use std::collections::HashMap;

pub mod end_turn;
pub mod finish_loading;
pub mod move_unit;
pub mod place_unit;
pub mod use_skill;

pub fn process_message(
    players: &mut HashMap<PlayerId, Player>,
    sender: PlayerId,
    game_data: &GameData,
    match_data: &mut MatchData,
    message: ClientToServerMessage,
) -> Result<Vec<ServerToClientMessageVariant>, ServerToClientMessage> {
    match message {
        ClientToServerMessage::FinishedLoading => {
            finish_loading::finish_loading(sender, players, match_data)
        }
        ClientToServerMessage::EndTurn => end_turn::end_turn(sender, match_data),
        ClientToServerMessage::PlaceUnit(data) => {
            place_unit::place_unit(sender, players, match_data, data)
        }
        ClientToServerMessage::MoveUnit(data) => move_unit::move_unit(sender, match_data, data),
        ClientToServerMessage::UseSkill(data) => {
            use_skill::use_skill(sender, match_data, data, game_data)
        }
        _ => Err(create_error_response(format!(
            "Unexpected message for server state InGame: {:?}",
            message
        ))),
    }
}
