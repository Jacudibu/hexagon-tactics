use crate::in_game_state::{PickUnitData, PlayerResources};
use crate::message_processor::command_invocation_result::{
    CommandInvocationResult, StateTransition,
};
use crate::message_processor::{create_error_response, ServerToClientMessageVariant};
use game_common::game_data::GameData;
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::server_to_client::{AddUnit, ServerToClientMessage};
use game_common::player::{Player, PlayerId};
use std::collections::HashMap;

pub fn process_message(
    sender: PlayerId,
    message: ClientToServerMessage,
    players: &mut HashMap<PlayerId, Player>,
    game_data: &GameData,
    player_resources: &mut HashMap<PlayerId, PlayerResources>,
    pick_unit_data: &mut PickUnitData,
) -> Result<CommandInvocationResult, ServerToClientMessage> {
    let ClientToServerMessage::PickUnit(pick_unit) = message else {
        todo!()
    };

    let Some(index) = pick_unit_data
        .units
        .iter()
        .position(|x| x.id == pick_unit.unit_id)
    else {
        return Err(create_error_response("Invalid unit id!"));
    };

    let mut unit = pick_unit_data.units.remove(index);
    unit.owner = sender;
    player_resources
        .get_mut(&sender)
        .unwrap()
        .units
        .push(unit.clone());

    let mut result = CommandInvocationResult::default().with_message(
        ServerToClientMessageVariant::SendToSender(ServerToClientMessage::AddUnit(AddUnit {
            unit,
        })),
    );

    if pick_unit_data.remaining_choices > 1 {
        result.set_state_transition(StateTransition::PickUnit {
            remaining: pick_unit_data.remaining_choices - 1,
        });
    } else {
        result.set_state_transition(StateTransition::StartCombat);
    }

    Ok(result)
}
