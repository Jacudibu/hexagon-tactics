pub mod combat;
pub mod combat_finished;
pub mod pick_unit;
mod waiting_for_others;

use crate::in_game::command_invocation_result::CommandInvocationResult;
use crate::in_game::in_game_data::{InGameData, StateData};
use crate::in_game::states::combat::{CombatState, CombatStateTransition};
use crate::in_game::states::combat_finished::{CombatFinishedState, CombatFinishedTransition};
use crate::in_game::states::pick_unit::{PickUnitState, PickUnitStateTransition};
use crate::in_game::states::waiting_for_others::{WaitingForOthers, WaitingForOthersTransition};
use crate::message_processor::ServerToClientMessageVariant;
use game_common::game_data::GameData;
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::server_to_client::ServerToClientMessage;
use game_common::player::{Player, PlayerId};
use hashbrown::HashMap;

pub enum StateTransitionKind {
    WaitingForOthers(WaitingForOthersTransition),
    Combat(CombatStateTransition),
    CombatFinished(CombatFinishedTransition),
    PickUnit(PickUnitStateTransition),
}

impl StateTransitionKind {
    #[must_use]
    pub fn on_state_enter(
        &self,
        in_game_data: &mut InGameData,
        affected_players: Vec<PlayerId>,
    ) -> Vec<ServerToClientMessageVariant> {
        match self {
            StateTransitionKind::PickUnit(transition) => {
                transition.execute(in_game_data, affected_players)
            }
            StateTransitionKind::Combat(transition) => {
                transition.execute(in_game_data, affected_players)
            }
            StateTransitionKind::CombatFinished(transition) => {
                transition.execute(in_game_data, affected_players)
            }
            StateTransitionKind::WaitingForOthers(transition) => {
                transition.execute(in_game_data, affected_players)
            }
        }
    }
}

pub enum InGameState {
    StartingGame,
    WaitingForOthers(WaitingForOthers),
    PickUnit(PickUnitState),
    Combat(CombatState),
    CombatFinished(CombatFinishedState),
}

impl InGameState {
    pub fn process_message(
        &mut self,
        sender: PlayerId,
        message: ClientToServerMessage,
        players: &mut HashMap<PlayerId, Player>,
        state_data: &mut StateData,
        game_data: &GameData,
    ) -> Result<CommandInvocationResult, ServerToClientMessage> {
        match self {
            InGameState::StartingGame => {
                // Technically this should never happen, as this is just the dummy initialization value
                todo!()
            }
            InGameState::WaitingForOthers(ref mut _state) => {
                // Until we have some kind of "stop waiting" command, this shouldn't be reachable
                todo!()
            }
            InGameState::Combat(ref mut state) => state.on_message(
                sender,
                message,
                players,
                state_data.player_resources,
                game_data,
            ),
            InGameState::CombatFinished(ref mut state) => {
                state.on_message(sender, message, state_data.player_resources)
            }
            InGameState::PickUnit(ref mut state) => state.on_message(sender, message, state_data),
        }
    }
}
