use crate::message_processor::{create_error_response, ServerToClientMessageVariant};
use crate::shared_state::SharedState;
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::server_to_client::ServerToClientMessage;
use game_common::player::PlayerId;

pub mod start_game;

pub fn process_message(
    shared_state: &mut SharedState,
    sender: PlayerId,
    message: ClientToServerMessage,
) -> Result<Vec<ServerToClientMessageVariant>, ServerToClientMessage> {
    match message {
        ClientToServerMessage::StartGame => start_game::start_game(shared_state),
        _ => Err(create_error_response(format!(
            "Unexpected message for server state WaitingForConnection: {:?}",
            message
        ))),
    }
}
