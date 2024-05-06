use crate::in_game_state::MatchData;
use crate::message_processor::ServerToClientMessageVariant;
use game_common::network_events::server_to_client::{ServerToClientMessage, StartUnitTurn};
use game_common::player::PlayerId;
use game_common::validation;

pub fn end_turn(
    sender: PlayerId,
    match_data: &mut MatchData,
) -> Result<Vec<ServerToClientMessageVariant>, ServerToClientMessage> {
    validation::validate_turn_order(sender, &match_data.combat_data)?;
    // TODO: Test

    let unit_id = match_data.combat_data.get_next_unit();
    match_data.combat_data.start_unit_turn(unit_id);
    Ok(vec![ServerToClientMessageVariant::Broadcast(
        ServerToClientMessage::StartUnitTurn(StartUnitTurn { unit_id }),
    )])
}