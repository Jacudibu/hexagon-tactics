use crate::state::ServerState::InGame;
use crate::state::{ServerData, ServerState, SharedState};
use game_common::game_map::GameMap;
use game_common::game_state::CombatData;
use game_common::network_events::client_to_server::*;
use game_common::network_events::server_to_client::*;
use game_common::network_events::{client_to_server, server_to_client, CONSTANT_LOCAL_PLAYER_ID};
use game_common::units::Unit;
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
        ClientToServerMessage::KeepAlive => Ok(Vec::new()),
        ClientToServerMessage::StartGame => start_game(shared_state),
        ClientToServerMessage::FinishedLoading => finish_loading(shared_state),
        ClientToServerMessage::PlaceUnit(data) => place_unit(shared_state, data),
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
        units_that_can_still_be_placed: Default::default(),
    };
    let server_data = ServerData {
        combat_state,
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
            error!("Wrong server state to receive FinishLoading events!")
        }
        InGame(ref mut server_data) => {
            server_data
                .combat_state
                .units_that_can_still_be_placed
                .push(unit_a.id);
            server_data
                .combat_state
                .units_that_can_still_be_placed
                .push(unit_b.id);
            server_data
                .combat_state
                .units_that_can_still_be_placed
                .push(unit_c.id);

            server_data
                .combat_state
                .units
                .insert(unit_a.id, unit_a.clone());
            server_data
                .combat_state
                .units
                .insert(unit_b.id, unit_b.clone());
            server_data
                .combat_state
                .units
                .insert(unit_c.id, unit_c.clone());
        }
    }

    // TODO: Check if all players are ready
    // TODO: Determine who starts
    Ok(vec![
        ServerToClientMessageVariant::Broadcast(ServerToClientMessage::PlayerIsReady(
            PlayerIsReady { player_id },
        )),
        ServerToClientMessageVariant::Broadcast(ServerToClientMessage::AddUnitToPlayer(
            AddUnitToPlayer { unit: unit_a },
        )),
        ServerToClientMessageVariant::Broadcast(ServerToClientMessage::AddUnitToPlayer(
            AddUnitToPlayer { unit: unit_b },
        )),
        ServerToClientMessageVariant::Broadcast(ServerToClientMessage::AddUnitToPlayer(
            AddUnitToPlayer { unit: unit_c },
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

    // TODO: Validation

    server_data
        .combat_state
        .unit_positions
        .insert(data.hex, data.unit_id);

    // TODO: Check if all units have been placed, and if so, proceed to very first unit turn
    let next = ServerToClientMessageVariant::Broadcast(
        ServerToClientMessage::PlayerTurnToPlaceUnit(PlayerTurnToPlaceUnit {
            player: CONSTANT_LOCAL_PLAYER_ID,
        }),
    );

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
