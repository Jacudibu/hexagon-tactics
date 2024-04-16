use crate::message_processor::ServerToClientMessageVariant;
use crate::state::MatchData;
use game_common::network_events::server_to_client::{ServerToClientMessage, StartUnitTurn};

pub fn end_turn(
    match_data: &mut MatchData,
) -> Result<Vec<ServerToClientMessageVariant>, ServerToClientMessage> {
    // TODO: Validate
    // TODO: Test

    let unit_id = match_data.combat_data.get_next_unit();
    match_data.combat_data.start_unit_turn(unit_id);
    Ok(vec![ServerToClientMessageVariant::Broadcast(
        ServerToClientMessage::StartUnitTurn(StartUnitTurn { unit_id }),
    )])
}
