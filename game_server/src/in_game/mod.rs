use crate::in_game::in_game_data::InGameData;
use crate::in_game::states::InGameState;
use crate::message_processor::ServerToClientMessageVariant;
use crate::shared_state::SharedState;
use game_common::game_data::GameData;
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::server_to_client::ServerToClientMessage;
use game_common::player::{Player, PlayerId};
use hashbrown::HashMap;

mod command_invocation_result;
pub mod in_game_data;
pub mod state_transitions;
pub mod states;

pub fn process_message(
    sender: PlayerId,
    message: ClientToServerMessage,
    game_data: &GameData,
    players: &mut HashMap<PlayerId, Player>,
    in_game_data: &mut InGameData,
) -> Result<Result<Vec<ServerToClientMessageVariant>, ServerToClientMessage>, ServerToClientMessage>
{
    let (player_state, player_resources) = in_game_data.deconstruct_for_processing(&sender);
    let mut result = match player_state {
        InGameState::StartingGame => {
            // Technically this should never happen, as this is just the dummy initialization value
            todo!()
        }
        InGameState::Combat(ref mut state) => {
            state.on_message(sender, message, players, player_resources, game_data)
        }
        InGameState::CombatFinished(ref mut state) => {
            state.on_message(sender, message, player_resources)
        }
        InGameState::PickUnit(ref mut state) => state.on_message(sender, message, player_resources),
    }?;

    if let Some(state_transition) = &result.state_transition {
        let mut new_messages =
            state_transitions::on_state_enter(&sender, state_transition, in_game_data);

        result.add_messages(&mut new_messages);
    }

    Ok(Ok(result.messages))
}
