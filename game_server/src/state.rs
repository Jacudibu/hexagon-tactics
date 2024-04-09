use game_common::game_state::GameState;
use game_common::network_events::server_to_client::ServerToClientMessage;
use game_common::network_events::NetworkMessage;
use std::collections::HashMap;
use std::io;
use std::sync::Arc;
use tokio::sync::{mpsc, Mutex};
use tracing::error;
use wtransport::Connection;

#[derive(Default)]
pub enum ServerState {
    #[default]
    WaitingForConnection,
    InGame(GameState),
}

#[derive(Default)]
pub struct SharedState {
    pub connections: HashMap<usize, mpsc::UnboundedSender<Vec<u8>>>,
    pub server_state: ServerState,
}

impl SharedState {
    pub fn broadcast(&mut self, message: ServerToClientMessage) {
        match message.serialize() {
            Ok(bytes) => {
                for (_, tx) in self.connections.iter_mut() {
                    // Wonder if there's some way of doing this without cloning?
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

    pub fn send_to(&mut self, sender: &usize, message: ServerToClientMessage) {
        match message.serialize() {
            Ok(bytes) => match self.connections.get(&sender) {
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
        exception: &usize,
        message: ServerToClientMessage,
    ) {
        match message.serialize() {
            Ok(bytes) => {
                for (addr, tx) in self.connections.iter_mut() {
                    if addr != exception {
                        // Wonder if there's some way of doing this without cloning?
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

pub struct ConnectedClient {
    pub rx: mpsc::UnboundedReceiver<Vec<u8>>,
    pub id: usize, // TODO: Use the id the server assigns when the connection is initialized in main instead
}

impl ConnectedClient {
    pub async fn new(
        state: Arc<Mutex<SharedState>>,
        connection: &Connection,
    ) -> io::Result<ConnectedClient> {
        let (tx, rx) = mpsc::unbounded_channel();
        let id = connection.stable_id();
        state.lock().await.connections.insert(id, tx);
        Ok(ConnectedClient { id, rx })
    }
}
