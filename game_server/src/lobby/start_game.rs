use game_common::network_events::server_to_client::{ServerToClientMessage, StartGame};
use game_common::player::{PlayerId, ReadyState};
use game_common::validation;

use crate::in_game::in_game_data::InGameData;
use crate::in_game::states::pick_unit::PickUnitStateTransition;
use crate::in_game::states::StateTransitionKind;
use crate::message_processor::ServerToClientMessageVariant;
use crate::server_state::ServerState;
use crate::shared_state::SharedState;

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
    for (id, _) in &shared_state.players {
        let start_state = StateTransitionKind::PickUnit(PickUnitStateTransition { remaining: 3 });
        // for message in start_state
        //     .on_state_enter(id, &mut in_game_data)
        //     .into_iter()
        // {
        //     if let ServerToClientMessageVariant::SendToSender(message) = message {
        //         messages.push(ServerToClientMessageVariant::SendTo((id.clone(), message)))
        //     } else {
        //         messages.push(message);
        //     }
        // }
        messages.append(&mut start_state.on_state_enter(id, &mut in_game_data))
    }

    shared_state.server_state = ServerState::InGame(in_game_data);
    Ok(messages)
}
