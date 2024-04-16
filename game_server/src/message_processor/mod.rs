use crate::state::{ServerState, SharedState};
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::server_to_client::{
    ErrorWhenProcessingMessage, ServerToClientMessage,
};

mod end_turn;
mod finish_loading;
mod move_unit;
mod place_unit;
mod start_game;

#[derive(Debug)]
pub enum ServerToClientMessageVariant {
    SendToSender(ServerToClientMessage),
    SendToEveryoneExceptSender(ServerToClientMessage),
    Broadcast(ServerToClientMessage),
}

pub fn process_message(
    shared_state: &mut SharedState,
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
        ServerState::InGame(ref mut server_data) => match message {
            ClientToServerMessage::FinishedLoading => finish_loading::finish_loading(server_data),
            ClientToServerMessage::EndTurn => end_turn::end_turn(server_data),
            ClientToServerMessage::PlaceUnit(data) => place_unit::place_unit(server_data, data),
            ClientToServerMessage::MoveUnit(data) => move_unit::move_unit(server_data, data),
            _ => Err(create_error_response(format!(
                "Unexpected message for server state InGame: {:?}",
                message
            ))),
        },
    }
}

fn create_error_response<T: Into<String>>(message: T) -> ServerToClientMessage {
    ServerToClientMessage::ErrorWhenProcessingMessage(ErrorWhenProcessingMessage {
        message: message.into(),
    })
}
