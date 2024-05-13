use crate::in_game_state::MatchData;
use crate::message_processor::command_invocation_result::CommandInvocationResult;
use crate::message_processor::{create_error_response, ServerToClientMessageVariant};
use game_common::combat_unit::{ActorId, CombatUnit, UnitId};
use game_common::game_data::GameData;
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::server_to_client::{CombatFinished, ServerToClientMessage};
use game_common::player::{Player, PlayerId};
use game_common::player_resources::PlayerResources;
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
    let mut check_win_conditions = true;
    let mut messages = match message {
        ClientToServerMessage::FinishedLoading => {
            check_win_conditions = false;
            finish_loading::finish_loading(sender, players, match_data)
        }
        ClientToServerMessage::EndTurn => end_turn::end_turn(sender, match_data),
        ClientToServerMessage::PlaceUnit(message) => {
            check_win_conditions = false;
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

    if check_win_conditions {
        let (player_units, ai_units) = count_alive_units(&match_data.combat_data.units);
        if player_units == 0 {
            messages.push(ServerToClientMessageVariant::Broadcast(
                ServerToClientMessage::CombatFinished(CombatFinished {
                    winner: ActorId::AI,
                }),
            ))
            // TODO: Transition
        } else if ai_units == 0 {
            messages.push(ServerToClientMessageVariant::Broadcast(
                ServerToClientMessage::CombatFinished(CombatFinished {
                    winner: ActorId::Player(sender), // TODO: Sender might still be losing if this is pvp
                }),
            ))
            // TODO: Transition
        }
    }

    Ok(CommandInvocationResult {
        state_transition: None,
        messages,
    })
}

fn count_alive_units(units: &HashMap<UnitId, CombatUnit>) -> (u32, u32) {
    // TODO: right now this quick & easy approach only works in co-operative scenarios
    let mut player_units = 0;
    let mut ai_units = 0;
    for unit in units.values() {
        if unit.is_dead() {
            continue;
        } else {
            if unit.owner == ActorId::AI {
                ai_units += 1;
            } else {
                player_units += 1;
            }
        }
    }

    (player_units, ai_units)
}
