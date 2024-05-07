use crate::shared_state::{ServerState, SharedState};
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::server_to_client::{
    ErrorWhenProcessingMessage, ServerToClientMessage,
};
use game_common::player::PlayerId;

mod combat;
mod lobby;
mod pick_unit;
pub mod state_transitions;

use crate::in_game_state::InGameState;
#[cfg(test)]
use enum_as_inner::EnumAsInner;

#[derive(Debug)]
#[cfg_attr(test, derive(EnumAsInner))]
pub enum ServerToClientMessageVariant {
    SendToSender(ServerToClientMessage),
    SendToEveryoneExceptSender(ServerToClientMessage),
    Broadcast(ServerToClientMessage),
}

pub fn process_message(
    shared_state: &mut SharedState,
    sender: PlayerId,
    message: ClientToServerMessage,
) -> Result<Vec<ServerToClientMessageVariant>, ServerToClientMessage> {
    match &mut shared_state.server_state {
        ServerState::Lobby => lobby::process_message(shared_state, sender, message),
        ServerState::InGame(ref mut in_game_data) => {
            let players = &mut shared_state.players;
            let game_data = &shared_state.game_data;
            let (player_state, player_resources) = in_game_data.deconstruct_for_processing(&sender);
            let (state_transition, mut result) = match player_state {
                InGameState::StartingGame => {
                    // Technically this should never happen, as this is just the dummy initialization value
                    todo!()
                }
                InGameState::Combat(ref mut match_data) => {
                    combat::process_message(players, sender, game_data, match_data, message)
                }
                InGameState::PickUnit(ref mut pick_unit_data) => pick_unit::process_message(
                    players,
                    sender,
                    game_data,
                    player_resources,
                    pick_unit_data,
                    message,
                ),
            }?;

            if let Some(state_transition) = state_transition {
                let mut new_messages =
                    state_transitions::handle_transition(&sender, state_transition, in_game_data);

                result.append(&mut new_messages);
            }

            Ok(result)
        }
    }
}

fn create_error_response<T: Into<String>>(message: T) -> ServerToClientMessage {
    ServerToClientMessage::ErrorWhenProcessingMessage(ErrorWhenProcessingMessage {
        message: message.into(),
    })
}
