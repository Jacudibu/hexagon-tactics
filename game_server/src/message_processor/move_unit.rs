use crate::message_processor::{create_error_response, ServerToClientMessageVariant};
use crate::state::ServerState::InGame;
use crate::state::SharedState;
use game_common::network_events::server_to_client::ServerToClientMessage;
use game_common::network_events::{client_to_server, server_to_client};
use tracing::error;

pub fn move_unit(
    shared_state: &mut SharedState,
    data: client_to_server::MoveUnit,
) -> Result<Vec<ServerToClientMessageVariant>, ServerToClientMessage> {
    // TODO: Consider moving state validation out. We don't need the whole shared state in these command processors.
    let InGame(server_data) = &mut shared_state.server_state else {
        error!("Something just went horribly wrong");
        return Err(create_error_response(
            "Something just went horribly wrong, yay!",
        ));
    };

    if data.path.is_empty() {
        // TODO: Throw Validation Error
        return Err(create_error_response("Path was empty!"));
    }

    // TODO: Validate
    // TODO: Test

    let unit = server_data
        .combat_data
        .units
        .get_mut(&server_data.combat_data.current_unit_turn.expect("TODO"))
        .expect("TODO");

    server_data
        .combat_data
        .unit_positions
        .remove(&unit.position);
    unit.position = data.path.last().unwrap().clone();
    server_data
        .combat_data
        .unit_positions
        .insert(unit.position, unit.id);

    Ok(vec![ServerToClientMessageVariant::Broadcast(
        ServerToClientMessage::MoveUnit(server_to_client::MoveUnit { path: data.path }),
    )])
}
