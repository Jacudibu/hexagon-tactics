use crate::message_processor::{create_error_response, ServerToClientMessageVariant};
use crate::state::ServerState::InGame;
use crate::state::SharedState;
use game_common::network_events::server_to_client::{ServerToClientMessage, StartUnitTurn};
use tracing::error;

pub fn end_turn(
    shared_state: &mut SharedState,
) -> Result<Vec<ServerToClientMessageVariant>, ServerToClientMessage> {
    // TODO: Consider moving state validation out. We don't need the whole shared state in these command processors.
    let InGame(server_data) = &mut shared_state.server_state else {
        error!("Something just went horribly wrong");
        return Err(create_error_response(
            "Something just went horribly wrong, yay!",
        ));
    };

    // TODO: Validate
    // TODO: Test

    let unit_id = server_data.combat_data.get_next_unit();
    server_data.combat_data.start_unit_turn(unit_id);
    Ok(vec![ServerToClientMessageVariant::Broadcast(
        ServerToClientMessage::StartUnitTurn(StartUnitTurn { unit_id }),
    )])
}
