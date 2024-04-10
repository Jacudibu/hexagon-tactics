use bytes::Bytes;
use game_common::game_map::GameMap;
use game_common::game_state::CombatData;
use game_common::network_events::server_to_client::ServerToClientMessage;
use game_common::network_events::NetworkMessage;
use std::collections::HashMap;
use tokio::sync::mpsc;
use tracing::error;

#[derive(Default)]
pub enum ServerState {
    #[default]
    WaitingForConnection,
    InGame(ServerData),
}

pub struct ServerData {
    pub loaded_map: GameMap,
    pub combat_state: CombatData,
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

pub type ClientId = u32;
pub struct ConnectedClient {
    pub receiver: mpsc::UnboundedReceiver<Bytes>,
    pub id: ClientId,
}
