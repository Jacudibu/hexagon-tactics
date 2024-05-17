use crate::in_game::command_invocation_result::StateTransition;
use crate::in_game::in_game_data::InGameData;
use crate::message_processor::ServerToClientMessageVariant;
use game_common::game_data::GameData;
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::server_to_client::ServerToClientMessage;
use game_common::player::{Player, PlayerId};
use hashbrown::HashMap;

mod command_invocation_result;
pub mod in_game_data;
pub mod states;

pub fn process_message(
    sender: PlayerId,
    message: ClientToServerMessage,
    game_data: &GameData,
    players: &mut HashMap<PlayerId, Player>,
    in_game_data: &mut InGameData,
) -> Result<Vec<ServerToClientMessageVariant>, ServerToClientMessage> {
    let (state, mut state_data) = in_game_data.deconstruct_for_processing(&sender);
    let result = state.process_message(sender, message, players, &mut state_data, game_data)?;

    drop(state_data);

    let result = process_state_transitions(in_game_data, result.messages, result.state_transitions);

    Ok(result)
}

fn process_state_transitions(
    in_game_data: &mut InGameData,
    mut messages: Vec<ServerToClientMessageVariant>,
    state_transitions: Vec<StateTransition>,
) -> Vec<ServerToClientMessageVariant> {
    for state_transition in state_transitions {
        let mut new_messages = state_transition
            .kind
            .on_state_enter(in_game_data, state_transition.players);

        messages.append(&mut new_messages);
    }

    messages
}
