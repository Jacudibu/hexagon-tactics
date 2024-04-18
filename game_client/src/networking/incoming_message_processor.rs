use crate::networking::network_plugin::{NetworkState, ServerConnection};
use bevy::app::{App, Plugin, PreUpdate};
use bevy::log::{debug, error};
use bevy::prelude::{in_state, EventWriter, IntoSystemConfigs, Local, ResMut};
use game_common::network_events::server_to_client::ServerToClientMessage;
use game_common::network_events::{server_to_client, NetworkMessage};

pub struct IncomingMessageProcessorPlugin;
impl Plugin for IncomingMessageProcessorPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<server_to_client::StartGameAndLoadMap>()
            .add_event::<server_to_client::PlayerIsReady>()
            .add_event::<server_to_client::AddUnitToPlayerStorage>()
            .add_event::<server_to_client::PlayerTurnToPlaceUnit>()
            .add_event::<server_to_client::PlaceUnit>()
            .add_event::<server_to_client::StartUnitTurn>()
            .add_event::<server_to_client::MoveUnit>()
            .add_event::<server_to_client::YouConnected>()
            .add_event::<server_to_client::OtherPlayerConnected>()
            .add_systems(
                PreUpdate,
                receive_updates.run_if(in_state(NetworkState::Connected)),
            );
    }
}

fn receive_updates(
    mut connection: ResMut<ServerConnection>,
    mut event_queue: Local<IncomingNetworkEventQueue>,
    mut load_map_event_from_server: EventWriter<server_to_client::StartGameAndLoadMap>,
    mut player_is_ready: EventWriter<server_to_client::PlayerIsReady>,
    mut add_unit_to_player: EventWriter<server_to_client::AddUnitToPlayerStorage>,
    mut player_turn_to_place_unit: EventWriter<server_to_client::PlayerTurnToPlaceUnit>,
    mut place_unit: EventWriter<server_to_client::PlaceUnit>,
    mut start_unit_turn: EventWriter<server_to_client::StartUnitTurn>,
    mut move_unit: EventWriter<server_to_client::MoveUnit>,
    mut you_connected: EventWriter<server_to_client::YouConnected>,
    mut other_player_connected: EventWriter<server_to_client::OtherPlayerConnected>,
) {
    if let Ok(bytes) = connection.message_receiver.try_recv() {
        match ServerToClientMessage::deserialize(&bytes) {
            Ok(messages) => {
                debug!("Received {} bytes: {:?}", bytes.len(), messages);
                for message in messages {
                    event_queue.push(message);
                }
            }
            Err(e) => {
                error!(
                    "Failed deserializing NetworkMessage! Error: {:?} Bytes: {:?}",
                    e, bytes
                );
                return;
            }
        };
    }

    // Only process one event per frame, that way incoming messages are guaranteed to stay in order.
    if let Some(message) = event_queue.pop_front() {
        match message {
            ServerToClientMessage::ErrorWhenProcessingMessage(e) => {
                error!("Server responded with an error: {:?}", e);
            }

            ServerToClientMessage::YouConnected(event) => {
                you_connected.send(event);
            }
            ServerToClientMessage::OtherPlayerConnected(event) => {
                other_player_connected.send(event);
            }

            ServerToClientMessage::LoadMap(event) => {
                load_map_event_from_server.send(event);
            }
            ServerToClientMessage::PlayerIsReady(event) => {
                player_is_ready.send(event);
            }
            ServerToClientMessage::AddUnitToPlayer(event) => {
                add_unit_to_player.send(event);
            }
            ServerToClientMessage::PlayerTurnToPlaceUnit(event) => {
                player_turn_to_place_unit.send(event);
            }
            ServerToClientMessage::PlaceUnit(event) => {
                place_unit.send(event);
            }
            ServerToClientMessage::StartUnitTurn(event) => {
                start_unit_turn.send(event);
            }
            ServerToClientMessage::MoveUnit(event) => {
                move_unit.send(event);
            }
        };
    }
}

#[derive(Default)]
struct IncomingNetworkEventQueue {
    queue: Vec<ServerToClientMessage>,
}

impl IncomingNetworkEventQueue {
    pub fn push(&mut self, message: ServerToClientMessage) {
        self.queue.push(message);
    }

    pub fn pop_front(&mut self) -> Option<ServerToClientMessage> {
        if self.queue.is_empty() {
            None
        } else {
            Some(self.queue.remove(0))
        }
    }
}
