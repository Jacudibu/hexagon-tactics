use crate::message_processor::command_invocation_result::{
    CommandInvocationResult, StateTransition,
};
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::server_to_client::ServerToClientMessage;
use game_common::player::PlayerId;

pub fn process_message(
    _sender: PlayerId,
    _message: ClientToServerMessage,
) -> Result<CommandInvocationResult, ServerToClientMessage> {
    let mut result = CommandInvocationResult::default();
    result.set_state_transition(StateTransition::PickUnit { remaining: 0 });

    Ok(result)
}
