mod incoming_message_processor;
mod internal_event_handlers;
mod network;
mod network_plugin;

use bevy::prelude::Resource;
use bevy::utils::HashMap;
use game_common::combat_unit::Owner;
use game_common::player::{Player, PlayerId};
pub use network::Network; // TODO: Should probably not be public and instead communicate via events
pub use network_plugin::{NetworkPlugin, NetworkState};

#[derive(Resource)]
pub struct LocalPlayerId {
    pub id: PlayerId,
    pub owner: Owner,
}

#[derive(Resource)]
pub struct ConnectedPlayers {
    pub players: HashMap<PlayerId, Player>,
}

#[derive(Resource)]
pub struct ClientIsLobbyHost;
