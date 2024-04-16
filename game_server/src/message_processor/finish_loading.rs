use crate::message_processor::ServerToClientMessageVariant;
use crate::state::MatchData;
use game_common::network_events::server_to_client::{
    AddUnitToPlayerStorage, PlayerIsReady, PlayerTurnToPlaceUnit, ServerToClientMessage,
};
use game_common::network_events::CONSTANT_LOCAL_PLAYER_ID;
use game_common::unit::Unit;

pub fn finish_loading(
    match_data: &mut MatchData,
) -> Result<Vec<ServerToClientMessageVariant>, ServerToClientMessage> {
    let player_id = CONSTANT_LOCAL_PLAYER_ID;

    let unit_a = Unit::create_debug_unit(1, player_id, "Unit A".into());
    let unit_b = Unit::create_debug_unit(2, player_id, "Unit B".into());
    let unit_c = Unit::create_debug_unit(3, player_id, "Unit C".into());

    match_data.combat_data.unit_storage.push(unit_a.clone());
    match_data.combat_data.unit_storage.push(unit_b.clone());
    match_data.combat_data.unit_storage.push(unit_c.clone());

    // TODO: Check if all players are ready
    // TODO: Determine who starts
    Ok(vec![
        ServerToClientMessageVariant::Broadcast(ServerToClientMessage::PlayerIsReady(
            PlayerIsReady { player_id },
        )),
        ServerToClientMessageVariant::Broadcast(ServerToClientMessage::AddUnitToPlayer(
            AddUnitToPlayerStorage { unit: unit_a },
        )),
        ServerToClientMessageVariant::Broadcast(ServerToClientMessage::AddUnitToPlayer(
            AddUnitToPlayerStorage { unit: unit_b },
        )),
        ServerToClientMessageVariant::Broadcast(ServerToClientMessage::AddUnitToPlayer(
            AddUnitToPlayerStorage { unit: unit_c },
        )),
        ServerToClientMessageVariant::Broadcast(ServerToClientMessage::PlayerTurnToPlaceUnit(
            PlayerTurnToPlaceUnit {
                player: CONSTANT_LOCAL_PLAYER_ID,
            },
        )),
    ])
}
