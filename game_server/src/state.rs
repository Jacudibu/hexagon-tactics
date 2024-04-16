use crate::connection_handler::ClientId;
use bytes::Bytes;
use game_common::combat_data::CombatData;
use game_common::game_map::GameMap;
use game_common::network_events::server_to_client::ServerToClientMessage;
use game_common::network_events::NetworkMessage;
use game_common::player::PlayerId;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tracing::error;

#[derive(Default)]
pub enum ServerState {
    #[default]
    WaitingForConnection,
    InGame(MatchData),
}

pub struct MatchData {
    pub loaded_map: GameMap,
    pub combat_data: CombatData,
}

#[derive(Default)]
pub struct SharedState {
    pub connections: HashMap<ClientId, mpsc::UnboundedSender<Bytes>>,
    pub server_state: ServerState,
}

impl SharedState {
    pub fn broadcast(&mut self, message: ServerToClientMessage) {
        match message.serialize() {
            Ok(bytes) => {
                for (_, tx) in self.connections.iter_mut() {
                    let _ = tx.send(bytes.clone());
                }
            }
            Err(e) => {
                error!(
                    "Error when trying to serialize NetworkMessage {:?} - Error: {:?}",
                    message, e
                );
            }
        }
    }

    pub fn send_to(&mut self, sender: &ClientId, message: ServerToClientMessage) {
        match message.serialize() {
            Ok(bytes) => match self.connections.get(sender) {
                None => {
                    error!("Unable to send Response {:?} - Sender {:?} of message was not found inside the connections array?", message, sender)
                }
                Some(tx) => {
                    let _ = tx.send(bytes);
                }
            },
            Err(e) => {
                error!(
                    "Error when trying to serialize NetworkMessage {:?} - Error: {:?}",
                    message, e
                );
            }
        }
    }

    pub fn send_to_everyone_except_one(
        &mut self,
        id_to_ignore: &ClientId,
        message: ServerToClientMessage,
    ) {
        match message.serialize() {
            Ok(bytes) => {
                for (id, tx) in self.connections.iter_mut() {
                    if id != id_to_ignore {
                        let _ = tx.send(bytes.clone());
                    }
                }
            }
            Err(e) => {
                error!(
                    "Error when trying to serialize NetworkMessage {:?} - Error: {:?}",
                    message, e
                );
            }
        }
    }
}

/// One Client can seat multiple players. While Connections might get replaced due to disconnects,
/// ConnectedPlayer will persist throughout the game, and their assigned client_id might change.
pub struct NetworkPlayer {
    client_id: ClientId,
    player_id: PlayerId,
    sender: mpsc::UnboundedSender<Bytes>,
    connection_state: ConnectionState,
}

enum ConnectionState {
    Connected,
    Disconnected,
}
