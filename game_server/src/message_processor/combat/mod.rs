use crate::in_game_state::MatchData;
use crate::message_processor::command_invocation_result::CommandInvocationResult;
use crate::message_processor::create_error_response;
use game_common::combat_unit::{CombatUnit, Owner, UnitId};
use game_common::game_data::GameData;
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::server_to_client::ServerToClientMessage;
use game_common::player::{Player, PlayerId};
use game_common::player_resources::PlayerResources;
use hashbrown::hash_map::Values;
use hashbrown::HashMap;

pub mod end_turn;
pub mod finish_loading;
pub mod move_unit;
pub mod place_unit;
pub mod use_skill;

pub fn process_message(
    sender: PlayerId,
    message: ClientToServerMessage,
    players: &mut HashMap<PlayerId, Player>,
    player_resources: &HashMap<PlayerId, PlayerResources>,
    game_data: &GameData,
    match_data: &mut MatchData,
) -> Result<CommandInvocationResult, ServerToClientMessage> {
    let result = match message {
        ClientToServerMessage::FinishedLoading => {
            finish_loading::finish_loading(sender, players, match_data)
        }
        ClientToServerMessage::EndTurn => end_turn::end_turn(sender, match_data),
        ClientToServerMessage::PlaceUnit(message) => {
            place_unit::place_unit(sender, message, players, player_resources, match_data)
        }
        ClientToServerMessage::MoveUnit(message) => {
            move_unit::move_unit(sender, message, match_data)
        }
        ClientToServerMessage::UseSkill(message) => {
            use_skill::use_skill(sender, message, match_data, game_data)
        }
        _ => Err(create_error_response(format!(
            "Unexpected message for server state InGame: {:?}",
            message
        ))),
    }?;

    Ok(CommandInvocationResult {
        state_transition: None,
        messages: result,
    })
}
