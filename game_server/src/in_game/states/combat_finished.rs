use crate::in_game::command_invocation_result::CommandInvocationResult;
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
    pub fn on_state_enter(
        &self,
        in_game_data: &mut InGameData,
        players_in_state: Vec<PlayerId>,
    ) -> Vec<ServerToClientMessageVariant> {
        for player in players_in_state {
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
        _sender: PlayerId,
        _message: ClientToServerMessage,
        _player_resources: &mut HashMap<PlayerId, PlayerResources>,
    ) -> Result<CommandInvocationResult, ServerToClientMessage> {
        let mut result = CommandInvocationResult::default();
        result.set_state_transition(StateTransitionKind::PickUnit(PickUnitStateTransition {
            remaining: 1,
        }));

        Ok(result)
    }
}
