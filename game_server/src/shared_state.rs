use crate::connection_handler::ConnectionId;
use crate::server_state::ServerState;
use bytes::Bytes;
use game_common::game_data::GameData;
use game_common::network_events::server_to_client::{
    OtherPlayerConnected, ServerToClientMessage, YouConnected,
};
use game_common::network_events::NetworkMessage;
use game_common::player::{Player, PlayerId, ReadyState};
use hashbrown::HashMap;
use tokio::sync::mpsc;
use tracing::error;

pub struct SharedState {
    pub connections: HashMap<ConnectionId, mpsc::UnboundedSender<Bytes>>,
    pub game_data: GameData,
    pub players: HashMap<PlayerId, Player>,
    pub player_to_connection_map: HashMap<PlayerId, ConnectionId>,
    pub connection_to_player_map: HashMap<ConnectionId, PlayerId>, // We would want to allow multiple players from the same connection for local/split-screen multiplayer, tho for now that'd just be an extra headache I guess
    pub server_state: ServerState,
}

impl Default for SharedState {
    fn default() -> Self {
        Self {
            connections: Default::default(),
            game_data: GameData::load(),
            players: Default::default(),
            player_to_connection_map: Default::default(),
            connection_to_player_map: Default::default(),
            server_state: Default::default(),
        }
    }
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

    pub fn send_to(&mut self, sender: &ConnectionId, message: ServerToClientMessage) {
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
        id_to_ignore: &ConnectionId,
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

    pub fn add_player_and_notify(&mut self, connection_id: ConnectionId) -> PlayerId {
        let player_id = self.players.len() + 1;
        let player = Player {
            id: player_id,
            name: format!("Player {player_id}"),
            ready_state: ReadyState::ReadyInLobby,
        };

        self.players.insert(player_id, player.clone());
        self.player_to_connection_map
            .insert(player_id, connection_id);
        self.connection_to_player_map
            .insert(connection_id, player_id);

        self.send_to(
            &connection_id,
            ServerToClientMessage::YouConnected(YouConnected {
                player_id,
                connected_players: self.players.values().cloned().collect(),
            }),
        );
        self.send_to_everyone_except_one(
            &connection_id,
            ServerToClientMessage::OtherPlayerConnected(OtherPlayerConnected { player }),
        );
        player_id
    }
}
