pub mod combat;
pub mod combat_finished;
pub mod pick_unit;

use crate::in_game::command_invocation_result::CommandInvocationResult;
use crate::in_game::in_game_data::InGameData;
use crate::in_game::states::combat::{CombatState, CombatStateTransition};
use crate::in_game::states::combat_finished::{CombatFinishedState, CombatFinishedTransition};
use crate::in_game::states::pick_unit::{PickUnitState, PickUnitStateTransition};
use crate::message_processor::ServerToClientMessageVariant;
use game_common::game_data::GameData;
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::server_to_client::ServerToClientMessage;
use game_common::player::{Player, PlayerId};
use game_common::player_resources::PlayerResources;
use hashbrown::HashMap;

pub enum StateTransitionKind {
    Combat(CombatStateTransition),
    CombatFinished(CombatFinishedTransition),
    PickUnit(PickUnitStateTransition),
}

impl StateTransitionKind {
    #[must_use]
    pub fn on_state_enter(
        &self,
        sender: &PlayerId,
        in_game_data: &mut InGameData,
    ) -> Vec<ServerToClientMessageVariant> {
        let players_in_state = in_game_data.get_all_players_in_same_state(sender);

        match self {
            StateTransitionKind::PickUnit(state) => {
                state.on_state_enter(in_game_data, players_in_state)
            }
            StateTransitionKind::Combat(state) => {
                state.on_state_enter(in_game_data, players_in_state)
            }
            StateTransitionKind::CombatFinished(state) => {
                state.on_state_enter(in_game_data, players_in_state)
            }
        }
    }
}

pub enum InGameState {
    StartingGame,
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
        player_resources: &mut HashMap<PlayerId, PlayerResources>,
        game_data: &GameData,
    ) -> Result<CommandInvocationResult, ServerToClientMessage> {
        match self {
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
            InGameState::PickUnit(ref mut state) => {
                state.on_message(sender, message, player_resources)
            }
        }
    }
}
