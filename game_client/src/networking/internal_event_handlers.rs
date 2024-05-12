use bevy::log::{error, info};
use bevy::prelude::{Commands, EventReader, NextState, ResMut};
use bevy::utils::HashMap;
use game_common::combat_unit::Owner;

use game_common::network_events::server_to_client::{
    OtherPlayerConnected, UpdateReadyStateForPlayer, YouConnected,
};
use game_common::DESYNC_TODO_MESSAGE;

use crate::networking::{ClientIsLobbyHost, ConnectedPlayers, LocalPlayerId, NetworkState};

pub fn on_you_connected(
    mut commands: Commands,
    mut events: EventReader<YouConnected>,
    mut next_network_state: ResMut<NextState<NetworkState>>,
) {
    for x in events.read() {
        let mut all_players = HashMap::new();
        for x in x.connected_players.iter() {
            all_players.insert(x.id, x.clone());
        }

        commands.insert_resource(LocalPlayerId {
            id: x.player_id,
            owner: Owner::Player(x.player_id),
        });
        commands.insert_resource(ConnectedPlayers {
            players: all_players,
        });

        if x.connected_players.len() == 1 {
            commands.insert_resource(ClientIsLobbyHost)
        }

        next_network_state.set(NetworkState::Connected);
        info!("Authentication Successful, Networking setup complete.")
    }
}

pub fn on_other_player_connected(
    mut connected_players: ResMut<ConnectedPlayers>,
    mut events: EventReader<OtherPlayerConnected>,
) {
    for x in events.read() {
        connected_players
            .players
            .insert(x.player.id, x.player.clone());

        info!("Other player connected: {:?}", x.player);
    }
}

pub fn on_update_ready_state_for_player(
    mut connected_players: ResMut<ConnectedPlayers>,
    mut events: EventReader<UpdateReadyStateForPlayer>,
) {
    for x in events.read() {
        match connected_players.players.get_mut(&x.player_id) {
            None => {
                error!("Player with id {} not found!", x.player_id);
                error!(DESYNC_TODO_MESSAGE);
            }
            Some(player) => {
                player.ready_state = x.ready_state;
            }
        }
    }
}
