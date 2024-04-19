use crate::message_processor::{create_error_response, validation, ServerToClientMessageVariant};
use crate::state::MatchData;
use game_common::network_events::server_to_client::ServerToClientMessage;
use game_common::network_events::{client_to_server, server_to_client};
use game_common::player::PlayerId;

pub fn move_unit(
    sender: PlayerId,
    match_data: &mut MatchData,
    data: client_to_server::MoveUnit,
) -> Result<Vec<ServerToClientMessageVariant>, ServerToClientMessage> {
    validation::validate_turn_order(sender, &match_data.combat_data)?;

    if data.path.is_empty() {
        // TODO: Throw Validation Error
        return Err(create_error_response("Path was empty!"));
    }

    // TODO: Validate
    // TODO: Test

    let unit = match_data
        .combat_data
        .units
        .get_mut(&match_data.combat_data.current_turn.as_unit_turn().unit_id)
        .expect("TODO");

    match_data.combat_data.unit_positions.remove(&unit.position);
    unit.position = data.path.last().unwrap().clone();
    match_data
        .combat_data
        .unit_positions
        .insert(unit.position, unit.id);

    Ok(vec![ServerToClientMessageVariant::Broadcast(
        ServerToClientMessage::MoveUnit(server_to_client::MoveUnit { path: data.path }),
    )])
}
