use crate::message_processor::{validation, ServerToClientMessageVariant};
use crate::state::MatchData;
use game_common::network_events::server_to_client::{
    ErrorWhenProcessingMessage, PlayerTurnToPlaceUnit, ServerToClientMessage, StartUnitTurn,
};
use game_common::network_events::{client_to_server, server_to_client};
use game_common::player::{Player, PlayerId};
use std::collections::HashMap;
use tracing::error;

pub fn place_unit(
    sender: PlayerId,
    players: &HashMap<PlayerId, Player>,
    match_data: &mut MatchData,
    data: client_to_server::PlaceUnit,
) -> Result<Vec<ServerToClientMessageVariant>, ServerToClientMessage> {
    validation::validate_turn_order(sender, &match_data.combat_data)?;
    validation::validate_player_owns_unit_with_id(sender, data.unit_id, &match_data.combat_data)?;

    if !match_data
        .combat_data
        .can_unit_be_placed_on_tile(&data.hex, &match_data.loaded_map)
    {
        return Err(ServerToClientMessage::ErrorWhenProcessingMessage(
            ErrorWhenProcessingMessage {
                message: "Invalid Position!".into(),
            },
        ));
    }

    let Some(index) = match_data
        .combat_data
        .unit_storage
        .iter()
        .position(|x| x.id == data.unit_id)
    else {
        error!(
            "Was unable to find unit with id {} in unit storage!",
            data.unit_id
        );
        return Err(ServerToClientMessage::ErrorWhenProcessingMessage(
            ErrorWhenProcessingMessage {
                message: "Invalid Unit ID!".into(),
            },
        ));
    };

    let mut unit = match_data.combat_data.unit_storage.remove(index);
    unit.position = data.hex;
    match_data
        .combat_data
        .unit_positions
        .insert(data.hex, data.unit_id);
    match_data.combat_data.units.insert(unit.id, unit);

    let next = if match_data.combat_data.unit_storage.is_empty() {
        let unit_id = match_data.combat_data.get_next_unit();
        match_data.combat_data.start_unit_turn(unit_id);
        ServerToClientMessageVariant::Broadcast(ServerToClientMessage::StartUnitTurn(
            StartUnitTurn { unit_id },
        ))
    } else {
        // TODO: Better Turn Order
        let next_player_id = match_data.combat_data.unit_storage[0].owner;
        ServerToClientMessageVariant::Broadcast(ServerToClientMessage::PlayerTurnToPlaceUnit(
            PlayerTurnToPlaceUnit {
                player: next_player_id,
            },
        ))
    };

    Ok(vec![
        ServerToClientMessageVariant::Broadcast(ServerToClientMessage::PlaceUnit(
            server_to_client::PlaceUnit {
                unit_id: data.unit_id,
                hex: data.hex,
            },
        )),
        next,
    ])
}
