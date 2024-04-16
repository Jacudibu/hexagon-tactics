use crate::state::SharedState;
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
    match message {
        ClientToServerMessage::StartGame => start_game::start_game(shared_state),
        ClientToServerMessage::FinishedLoading => finish_loading::finish_loading(shared_state),
        ClientToServerMessage::EndTurn => end_turn::end_turn(shared_state),
        ClientToServerMessage::PlaceUnit(data) => place_unit::place_unit(shared_state, data),
        ClientToServerMessage::MoveUnit(data) => move_unit::move_unit(shared_state, data),
    }
}

fn create_error_response(message: &str) -> ServerToClientMessage {
    ServerToClientMessage::ErrorWhenProcessingMessage(ErrorWhenProcessingMessage {
        message: message.into(),
    })
}
