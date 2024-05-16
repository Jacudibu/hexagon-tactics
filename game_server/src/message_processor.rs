use crate::shared_state::{ServerState, SharedState};
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::server_to_client::{
    ErrorWhenProcessingMessage, ServerToClientMessage,
};
use game_common::player::PlayerId;

use crate::in_game::in_game_data::InGameData;
use crate::in_game::state_transitions;
use crate::in_game::states::InGameState;
use crate::{in_game, lobby};
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
            in_game::process_message(sender, message, game_data, players, in_game_data)?
        }
    }
}

pub fn create_error_response<T: Into<String>>(message: T) -> ServerToClientMessage {
    ServerToClientMessage::ErrorWhenProcessingMessage(ErrorWhenProcessingMessage {
        message: message.into(),
    })
}
