use crate::connection_handler::ConnectionId;
use bytes::Bytes;
use game_common::combat_data::CombatData;
use game_common::game_map::GameMap;
use game_common::network_events::server_to_client::{
    OtherPlayerConnected, ServerToClientMessage, YouConnected,
};
use game_common::network_events::NetworkMessage;
use game_common::player::{Player, PlayerId, ReadyState};
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
    pub connections: HashMap<ConnectionId, mpsc::UnboundedSender<Bytes>>,
    pub players: HashMap<PlayerId, Player>,
    pub player_to_connection_map: HashMap<PlayerId, ConnectionId>,
    pub connection_to_player_map: HashMap<ConnectionId, PlayerId>, // TODO: We would want to allow multiple players from the same connection for local multiplayer
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

/// One Client can seat multiple players. While Connections might get replaced due to disconnects,
/// ConnectedPlayer will persist throughout the game, and their assigned client_id might change.
pub struct NetworkPlayer {
    client_id: ConnectionId,
    player_id: PlayerId,
    sender: mpsc::UnboundedSender<Bytes>,
    connection_state: ConnectionState,
}

enum ConnectionState {
    Connected,
    Disconnected,
}
