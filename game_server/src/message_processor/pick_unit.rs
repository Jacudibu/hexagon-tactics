use crate::in_game_state::{PickUnitData, PlayerResources};
use crate::message_processor::state_transitions::StateTransition;
use crate::message_processor::{create_error_response, ServerToClientMessageVariant};
use game_common::game_data::GameData;
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::server_to_client::{ServerToClientMessage, StartGameAndLoadMap};
use game_common::player::{Player, PlayerId};
use game_common::TEST_MAP_NAME;
use std::collections::HashMap;

pub fn process_message(
    players: &mut HashMap<PlayerId, Player>,
    sender: PlayerId,
    game_data: &GameData,
    player_resources: &mut HashMap<PlayerId, PlayerResources>,
    pick_unit_data: &mut PickUnitData,
    message: ClientToServerMessage,
) -> Result<(Option<StateTransition>, Vec<ServerToClientMessageVariant>), ServerToClientMessage> {
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

    let unit = pick_unit_data.units.remove(index);
    player_resources.get_mut(&sender).unwrap().units.push(unit);

    // TODO: Send AddUnit command to player

    if pick_unit_data.remaining_choices > 0 {
        todo!()
    } else {
        Ok((
            Some(StateTransition::StartCombat),
            vec![ServerToClientMessageVariant::Broadcast(
                ServerToClientMessage::LoadMap(StartGameAndLoadMap {
                    path: TEST_MAP_NAME.into(),
                }),
            )],
        ))
    }
}
