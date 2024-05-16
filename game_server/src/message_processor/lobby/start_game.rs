use game_common::network_events::server_to_client::{ServerToClientMessage, StartGame};
use game_common::player::{PlayerId, ReadyState};
use game_common::validation;

use crate::in_game_state::InGameData;
use crate::message_processor::command_invocation_result::StateTransition;
use crate::message_processor::{state_transitions, ServerToClientMessageVariant};
use crate::shared_state::{ServerState, SharedState};

pub fn start_game(
    shared_state: &mut SharedState,
    sender: PlayerId,
) -> Result<Vec<ServerToClientMessageVariant>, ServerToClientMessage> {
    validation::validate_player_readiness(&shared_state.players, &ReadyState::ReadyInLobby)?;

    let mut messages = Vec::new();
    messages.push(ServerToClientMessageVariant::Broadcast(
        ServerToClientMessage::StartGame(StartGame {}),
    ));

    let mut in_game_data = InGameData::new(&shared_state);
    messages.append(&mut state_transitions::on_state_enter(
        &sender,
        &StateTransition::PickUnit { remaining: 3 },
        &mut in_game_data,
    ));

    shared_state.server_state = ServerState::InGame(in_game_data);
    Ok(messages)
}
