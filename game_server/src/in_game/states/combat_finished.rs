use crate::in_game::command_invocation_result::{CommandInvocationResult, StateTransition};
use crate::in_game::in_game_data::InGameData;
use crate::in_game::states::pick_unit::PickUnitStateTransition;
use crate::in_game::states::InGameState;
use crate::in_game::states::StateTransitionKind;
use crate::message_processor::ServerToClientMessageVariant;
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::server_to_client::ServerToClientMessage;
use game_common::player::PlayerId;
use game_common::player_resources::PlayerResources;
use hashbrown::HashMap;

pub struct CombatFinishedTransition;
pub struct CombatFinishedState;

impl CombatFinishedTransition {
    #[must_use]
    pub fn execute(
        &self,
        in_game_data: &mut InGameData,
        affected_players: Vec<PlayerId>,
    ) -> Vec<ServerToClientMessageVariant> {
        for player in affected_players {
            in_game_data.insert_state_for_player(
                player,
                InGameState::CombatFinished(CombatFinishedState {}),
            )
        }

        Vec::new()
    }
}

impl CombatFinishedState {
    pub fn on_message(
        &mut self,
        sender: PlayerId,
        _message: ClientToServerMessage,
        _player_resources: &mut HashMap<PlayerId, PlayerResources>,
    ) -> Result<CommandInvocationResult, ServerToClientMessage> {
        let mut result = CommandInvocationResult::default();
        result.add_state_transition(StateTransition::new(
            sender,
            StateTransitionKind::PickUnit(PickUnitStateTransition { remaining: 1 }),
        ));

        Ok(result)
    }
}
