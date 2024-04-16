use crate::state::ServerState::InGame;
use crate::state::{ServerData, ServerState, SharedState};
use game_common::combat_data::CombatData;
use game_common::game_map::GameMap;
use game_common::network_events::client_to_server::*;
use game_common::network_events::server_to_client::*;
use game_common::network_events::{client_to_server, server_to_client, CONSTANT_LOCAL_PLAYER_ID};
use game_common::unit::Unit;
use game_common::TEST_MAP_NAME;
use tracing::error;

#[derive(Debug)]
pub enum ServerToClientMessageVariant {
    SendToSender(ServerToClientMessage),
    SendToEveryoneExceptSender(ServerToClientMessage),
    Broadcast(ServerToClientMessage),
}

pub fn process_message(
    shared_state: &mut SharedState,
    message: ClientToServerMessage,
) -> Result<Vec<ServerToClientMessageVariant>, ServerToClientMessage> {
    match message {
        ClientToServerMessage::StartGame => start_game(shared_state),
        ClientToServerMessage::FinishedLoading => finish_loading(shared_state),
        ClientToServerMessage::PlaceUnit(data) => place_unit(shared_state, data),
        ClientToServerMessage::MoveUnit(data) => move_unit(shared_state, data),
    }
}

fn start_game(
    shared_state: &mut SharedState,
) -> Result<Vec<ServerToClientMessageVariant>, ServerToClientMessage> {
    let map = match GameMap::load_from_file(TEST_MAP_NAME) {
        Ok(map) => map,
        Err(_) => {
            return Err(ServerToClientMessage::ErrorWhenProcessingMessage(
                ErrorWhenProcessingMessage {
                    message: "Server failed to load map!".into(),
                },
            ))
        }
    };
    let combat_state = CombatData {
        units: Default::default(),
        unit_positions: Default::default(),
        turn_order: Default::default(),
        unit_storage: Default::default(),
        current_unit_turn: None,
        turn_resources: Default::default(),
    };
    let server_data = ServerData {
        combat_data: combat_state,
        loaded_map: map,
    };

    shared_state.server_state = InGame(server_data);

    Ok(vec![ServerToClientMessageVariant::Broadcast(
        ServerToClientMessage::LoadMap(StartGameAndLoadMap {
            path: TEST_MAP_NAME.into(),
        }),
    )])
}

fn finish_loading(
    shared_state: &mut SharedState,
) -> Result<Vec<ServerToClientMessageVariant>, ServerToClientMessage> {
    let player_id = CONSTANT_LOCAL_PLAYER_ID;

    let unit_a = Unit::create_debug_unit(1, player_id, "Unit A".into());
    let unit_b = Unit::create_debug_unit(2, player_id, "Unit B".into());
    let unit_c = Unit::create_debug_unit(3, player_id, "Unit C".into());

    match shared_state.server_state {
        ServerState::WaitingForConnection => {
            error!("Wrong server state to receive FinishLoading events!");
        }
        InGame(ref mut server_data) => {
            server_data.combat_data.unit_storage.push(unit_a.clone());
            server_data.combat_data.unit_storage.push(unit_b.clone());
            server_data.combat_data.unit_storage.push(unit_c.clone());
        }
    }

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

fn place_unit(
    shared_state: &mut SharedState,
    data: client_to_server::PlaceUnit,
) -> Result<Vec<ServerToClientMessageVariant>, ServerToClientMessage> {
    // TODO: Consider moving state validation out. We don't need the whole shared state in these command processors.
    let InGame(server_data) = &mut shared_state.server_state else {
        error!("Something just went horribly wrong");
        return Err(ServerToClientMessage::ErrorWhenProcessingMessage(
            ErrorWhenProcessingMessage {
                message: "Something just went horribly wrong, yay!".into(),
            },
        ));
    };

    if !server_data
        .combat_data
        .can_unit_be_placed_on_tile(&data.hex, &server_data.loaded_map)
    {
        return Err(ServerToClientMessage::ErrorWhenProcessingMessage(
            ErrorWhenProcessingMessage {
                message: "Invalid Position!".into(),
            },
        ));
    }

    let Some(index) = server_data
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

    let mut unit = server_data.combat_data.unit_storage.remove(index);
    unit.position = data.hex;
    server_data
        .combat_data
        .unit_positions
        .insert(data.hex, data.unit_id);
    server_data.combat_data.units.insert(unit.id, unit);

    // TODO: Check if all units have been placed, and if so, proceed to very first unit turn
    let next = if server_data.combat_data.unit_storage.is_empty() {
        let unit_id = server_data.combat_data.get_next_unit();
        server_data.combat_data.start_unit_turn(unit_id);
        ServerToClientMessageVariant::Broadcast(ServerToClientMessage::StartUnitTurn(
            StartUnitTurn { unit_id },
        ))
    } else {
        ServerToClientMessageVariant::Broadcast(ServerToClientMessage::PlayerTurnToPlaceUnit(
            PlayerTurnToPlaceUnit {
                player: CONSTANT_LOCAL_PLAYER_ID,
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

fn create_error_response(message: &str) -> ServerToClientMessage {
    ServerToClientMessage::ErrorWhenProcessingMessage(ErrorWhenProcessingMessage {
        message: message.into(),
    })
}

fn move_unit(
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
