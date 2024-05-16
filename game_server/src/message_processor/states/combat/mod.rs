use crate::in_game_data::InGameData;
use crate::message_processor::command_invocation_result::CommandInvocationResult;
use crate::message_processor::states::combat_finished::CombatFinishedTransition;
use crate::message_processor::states::InGameState;
use crate::message_processor::states::StateTransitionKind;
use crate::message_processor::{create_error_response, ServerToClientMessageVariant};
use game_common::combat_data::CombatData;
use game_common::combat_turn::CombatTurn;
use game_common::combat_unit::{ActorId, CombatUnit, UnitId};
use game_common::game_data::GameData;
use game_common::game_map::GameMap;
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::server_to_client::{
    CombatFinished, ErrorWhenProcessingMessage, LoadMap, ServerToClientMessage,
};
use game_common::player::{Player, PlayerId};
use game_common::player_resources::PlayerResources;
use game_common::TEST_MAP_NAME;
use hashbrown::HashMap;

pub mod end_turn;
pub mod finish_loading;
pub mod move_unit;
pub mod place_unit;
pub mod use_skill;

pub struct CombatStateTransition {}
pub struct CombatState {
    pub loaded_map: GameMap,
    pub combat_data: CombatData,
}

impl CombatStateTransition {
    #[must_use]
    pub fn on_state_enter(
        &self,
        in_game_data: &mut InGameData,
        players_in_state: Vec<PlayerId>,
    ) -> Vec<ServerToClientMessageVariant> {
        let mut result = Vec::new();
        let map = match GameMap::load_from_file(TEST_MAP_NAME) {
            Ok(map) => map,
            Err(_) => {
                // TODO: Load a "known good" map instead, and send player the correct map id
                result.push(ServerToClientMessageVariant::SendToSender(
                    ServerToClientMessage::ErrorWhenProcessingMessage(ErrorWhenProcessingMessage {
                        message: "Server failed to load map!".into(),
                    }),
                ));
                GameMap::new(2)
            }
        };
        let combat_data = CombatData {
            units: Default::default(),
            unit_positions: Default::default(),
            current_turn: CombatTurn::Undefined,
        };
        let match_data = CombatState {
            combat_data,
            loaded_map: map,
        };

        in_game_data.insert_state_for_player(players_in_state[0], InGameState::Combat(match_data));
        for player_id in 1..players_in_state.len() {
            in_game_data.add_player_to_other_player_state(&players_in_state[0], player_id.clone());
        }

        result.push(ServerToClientMessageVariant::Broadcast(
            ServerToClientMessage::LoadMap(LoadMap {
                path: TEST_MAP_NAME.into(),
            }),
        ));

        result
    }
}

impl CombatState {
    pub fn on_message(
        &mut self,
        sender: PlayerId,
        message: ClientToServerMessage,
        players: &mut HashMap<PlayerId, Player>,
        player_resources: &mut HashMap<PlayerId, PlayerResources>,
        game_data: &GameData,
    ) -> Result<CommandInvocationResult, ServerToClientMessage> {
        let mut check_win_conditions = true;
        let mut messages = match message {
            ClientToServerMessage::FinishedLoading => {
                check_win_conditions = false;
                finish_loading::finish_loading(sender, players, self)
            }
            ClientToServerMessage::EndTurn => end_turn::end_turn(sender, self),
            ClientToServerMessage::PlaceUnit(message) => {
                check_win_conditions = false;
                place_unit::place_unit(sender, message, players, player_resources, self)
            }
            ClientToServerMessage::MoveUnit(message) => move_unit::move_unit(sender, message, self),
            ClientToServerMessage::UseSkill(message) => {
                use_skill::use_skill(sender, message, self, game_data)
            }
            _ => Err(create_error_response(format!(
                "Unexpected message for server state InGame: {:?}",
                message
            ))),
        }?;

        let mut state_transition = None;
        if check_win_conditions {
            let (player_units, ai_units) = count_alive_units(&self.combat_data.units);
            if player_units == 0 {
                state_transition = Some(StateTransitionKind::CombatFinished(
                    CombatFinishedTransition {},
                ));
                messages.push(ServerToClientMessageVariant::Broadcast(
                    ServerToClientMessage::CombatFinished(CombatFinished {
                        winner: ActorId::AI,
                    }),
                ));
            } else if ai_units == 0 {
                state_transition = Some(StateTransitionKind::CombatFinished(
                    CombatFinishedTransition {},
                ));
                messages.push(ServerToClientMessageVariant::Broadcast(
                    ServerToClientMessage::CombatFinished(CombatFinished {
                        winner: ActorId::Player(sender), // TODO: Sender might still be losing if this is pvp
                    }),
                ));
            }
        }

        Ok(CommandInvocationResult {
            state_transition,
            messages,
        })
    }
}

fn count_alive_units(units: &HashMap<UnitId, CombatUnit>) -> (u32, u32) {
    // TODO: right now this quick & easy approach only works in co-operative scenarios
    let mut player_units = 0;
    let mut ai_units = 0;
    for unit in units.values() {
        if unit.is_dead() {
            continue;
        } else {
            if unit.owner == ActorId::AI {
                ai_units += 1;
            } else {
                player_units += 1;
            }
        }
    }

    (player_units, ai_units)
}
