use crate::state::{ServerState, SharedState};
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::server_to_client::{
    ErrorWhenProcessingMessage, ServerToClientMessage,
};
use game_common::player::PlayerId;

mod combat;
mod start_game;

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
        ServerState::WaitingForConnection => match message {
            ClientToServerMessage::StartGame => start_game::start_game(shared_state),
            _ => Err(create_error_response(format!(
                "Unexpected message for server state WaitingForConnection: {:?}",
                message
            ))),
        },
        ServerState::SelectingStartingUnits(ref mut data) => {
            todo!()
        }
        ServerState::InCombat(ref mut match_data) => {
            let players = &mut shared_state.players;
            let game_data = &shared_state.game_data;
            combat::process_message(players, sender, game_data, match_data, message)
        }
    }
}

fn create_error_response<T: Into<String>>(message: T) -> ServerToClientMessage {
    ServerToClientMessage::ErrorWhenProcessingMessage(ErrorWhenProcessingMessage {
        message: message.into(),
    })
}
