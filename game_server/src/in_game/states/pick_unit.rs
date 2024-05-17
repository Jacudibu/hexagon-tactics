use crate::in_game::command_invocation_result::{CommandInvocationResult, StateTransition};
use crate::in_game::in_game_data::{InGameData, StateData};
use crate::in_game::states::combat::CombatStateTransition;
use crate::in_game::states::waiting_for_others::WaitingForOthersTransition;
use crate::in_game::states::InGameState;
use crate::in_game::states::StateTransitionKind;
use crate::message_processor::{create_error_response, ServerToClientMessageVariant};
use game_common::combat_unit::get_unique_unit_id;
use game_common::game_data::unit_definition::UnitDefinition;
use game_common::game_data::DEBUG_RACE_ID;
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::server_to_client::{
    AddUnit, ChooseBetweenUnits, ServerToClientMessage,
};
use game_common::player::PlayerId;

pub struct PickUnitStateTransition {
    pub remaining: u8,
}

pub struct PickUnitState {
    units: Vec<UnitDefinition>,
    remaining_choices: u8,
}

impl PickUnitStateTransition {
    #[must_use]
    pub fn execute(
        &self,
        in_game_data: &mut InGameData,
        affected_players: Vec<PlayerId>,
    ) -> Vec<ServerToClientMessageVariant> {
        assert_eq!(
            1,
            affected_players.len(),
            "Pick unit states should always be unique for every player!"
        );
        let player = affected_players[0];

        let mut result = Vec::new();
        let state = PickUnitState {
            units: create_units(3),
            remaining_choices: self.remaining,
        };
        result.push(ServerToClientMessageVariant::SendTo((
            player,
            ServerToClientMessage::ChooseBetweenUnits(ChooseBetweenUnits {
                units: state.units.clone(),
            }),
        )));
        in_game_data.insert_state_for_player(player, InGameState::PickUnit(state));

        result
    }
}

impl PickUnitState {
    pub fn on_message(
        &mut self,
        sender: PlayerId,
        message: ClientToServerMessage,
        state_data: &mut StateData,
    ) -> Result<CommandInvocationResult, ServerToClientMessage> {
        let ClientToServerMessage::PickUnit(message) = message else {
            todo!()
        };

        let Some(index) = self.units.iter().position(|x| x.id == message.unit_id) else {
            return Err(create_error_response("Invalid unit id!"));
        };

        let mut unit = self.units.remove(index);
        unit.owner = sender;
        state_data
            .player_resources
            .get_mut(&sender)
            .unwrap()
            .units
            .push(unit.clone());

        let mut result = CommandInvocationResult::default().with_message(
            ServerToClientMessageVariant::SendToSender(ServerToClientMessage::AddUnit(AddUnit {
                unit,
            })),
        );

        if self.remaining_choices > 1 {
            result.add_state_transition(StateTransition::new(
                sender,
                StateTransitionKind::PickUnit(PickUnitStateTransition {
                    remaining: self.remaining_choices - 1,
                }),
            ));
        } else {
            let transition = if state_data.are_all_other_players_ready() {
                StateTransition {
                    players: state_data.all_player_ids(),
                    kind: StateTransitionKind::Combat(CombatStateTransition {}),
                }
            } else {
                StateTransition::new(
                    sender,
                    StateTransitionKind::WaitingForOthers(WaitingForOthersTransition {}),
                )
            };
            result.add_state_transition(transition);
        }

        Ok(result)
    }
}

fn create_units(amount: u8) -> Vec<UnitDefinition> {
    (0..amount).map(|_| create_unit()).collect()
}

fn create_unit() -> UnitDefinition {
    let id = get_unique_unit_id();
    UnitDefinition {
        id,
        owner: 0,
        name: format!("Unit #{}", id),
        race: DEBUG_RACE_ID,
        levels: Default::default(),
        unlocked_skills: vec![],
        weapon: None,
        armor: None,
        accessory: None,
    }
}
