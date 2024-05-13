use crate::in_game_state::MatchData;
use crate::message_processor::{create_error_response, ServerToClientMessageVariant};
use game_common::combat_turn::{CombatTurn, PlaceUnit};
use game_common::combat_unit::{ActorId, CombatUnit};
use game_common::network_events::server_to_client::{
    ErrorWhenProcessingMessage, PlayerTurnToPlaceUnit, ServerToClientMessage, StartUnitTurn,
};
use game_common::network_events::{client_to_server, server_to_client};
use game_common::player::{Player, PlayerId};
use game_common::player_resources::PlayerResources;
use game_common::validation;
use hashbrown::HashMap;

pub fn place_unit(
    sender: PlayerId,
    message: client_to_server::PlaceUnit,
    players: &HashMap<PlayerId, Player>,
    player_resources: &HashMap<PlayerId, PlayerResources>,
    match_data: &mut MatchData,
) -> Result<Vec<ServerToClientMessageVariant>, ServerToClientMessage> {
    validation::validate_turn_order(sender, &match_data.combat_data)?;
    let unit = validation::validate_player_owns_resource_unit_with_id(
        sender,
        message.unit_id,
        player_resources,
    )?;

    if !match_data
        .combat_data
        .can_unit_be_placed_on_tile(&message.hex, &match_data.loaded_map)
    {
        return Err(ServerToClientMessage::ErrorWhenProcessingMessage(
            ErrorWhenProcessingMessage {
                message: "Invalid Position!".into(),
            },
        ));
    }

    if match_data.combat_data.units.contains_key(&unit.id) {
        return Err(create_error_response("Unit has already been placed!"));
    }

    let mut unit = CombatUnit::from(unit);
    unit.position = message.hex;
    match_data
        .combat_data
        .unit_positions
        .insert(message.hex, message.unit_id);
    match_data.combat_data.units.insert(unit.id, unit.clone());

    // TODO: start combat when X units have been placed by each player instead of doing... this
    let next = if match_data.combat_data.units.len() >= (3 * players.len() + 2) {
        let unit_id = match_data.combat_data.get_unit_for_next_turn();
        match_data.combat_data.start_unit_turn(unit_id);
        ServerToClientMessageVariant::Broadcast(ServerToClientMessage::StartUnitTurn(
            StartUnitTurn { unit_id },
        ))
    } else {
        // TODO: Better Turn Order
        fn count_units(match_data: &MatchData, owner: &ActorId) -> usize {
            match_data
                .combat_data
                .units
                .iter()
                .filter(|(_, unit)| &unit.owner == owner)
                .count()
        }

        let next_player_id = players
            .keys()
            .min_by(|&&player_a, &&player_b| {
                count_units(match_data, &ActorId::Player(player_a))
                    .cmp(&count_units(match_data, &ActorId::Player(player_b)))
            })
            .unwrap();
        match_data.combat_data.current_turn = CombatTurn::PlaceUnit(PlaceUnit {
            player_id: next_player_id.clone(),
        });
        ServerToClientMessageVariant::Broadcast(ServerToClientMessage::PlayerTurnToPlaceUnit(
            PlayerTurnToPlaceUnit {
                player: next_player_id.clone(),
            },
        ))
    };

    Ok(vec![
        ServerToClientMessageVariant::Broadcast(ServerToClientMessage::PlaceUnit(
            server_to_client::PlaceUnit { unit },
        )),
        next,
    ])
}
