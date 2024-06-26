use crate::in_game::states::combat::CombatState;
use crate::message_processor::{create_error_response, ServerToClientMessageVariant};
use game_common::combat_turn::{CombatTurn, PlaceUnit};
use game_common::combat_unit::{ActorId, CombatUnit};
use game_common::game_data::GameData;
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
    combat_state: &mut CombatState,
    game_data: &GameData,
) -> Result<Vec<ServerToClientMessageVariant>, ServerToClientMessage> {
    validation::validate_turn_order(sender, &combat_state.combat_data)?;
    let unit = validation::validate_player_owns_resource_unit_with_id(
        sender,
        message.unit_id,
        player_resources,
    )?;

    validation::validate_unit_can_be_placed_on_tile(
        &combat_state.combat_data,
        &unit.id,
        &1,
        &message.hex,
        &combat_state.loaded_map,
    )?;

    let mut unit = CombatUnit::from_unit_definition(unit, game_data);
    unit.position = message.hex;
    combat_state
        .combat_data
        .unit_positions
        .insert(message.hex, message.unit_id);
    combat_state.combat_data.units.insert(unit.id, unit.clone());

    // TODO: start combat when X units have been placed by each player instead of doing... this
    // TODO: Also make sure that the user actually has any more units to place now that we can remove them
    let next = if combat_state.combat_data.units.len() >= (3 * players.len() + 2) {
        let unit_id = combat_state.combat_data.get_unit_for_next_turn();
        combat_state.combat_data.start_unit_turn(unit_id);
        ServerToClientMessageVariant::Broadcast(ServerToClientMessage::StartUnitTurn(
            StartUnitTurn { unit_id },
        ))
    } else {
        // TODO: Better Turn Order
        fn count_units(match_data: &CombatState, owner: &ActorId) -> usize {
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
                count_units(combat_state, &ActorId::Player(player_a))
                    .cmp(&count_units(combat_state, &ActorId::Player(player_b)))
            })
            .unwrap();
        combat_state.combat_data.current_turn = CombatTurn::PlaceUnit(PlaceUnit {
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
