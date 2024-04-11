use crate::networking::incoming_message_processor::IncomingMessageProcessorPlugin;
use crate::networking::network::Network;
use bevy::prelude::{
    error, in_state, info, not, on_event, App, Commands, EventReader, EventWriter,
    IntoSystemConfigs, NextState, Plugin, PostUpdate, PreUpdate, Res, ResMut, Resource, States,
    Timer, TimerMode,
};
use bevy::time::Time;
use bytes::Bytes;
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::{NetworkMessage, NETWORK_IDLE_TIMEOUT};
use tokio::sync::mpsc;

pub struct NetworkPlugin;
impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(Network::default())
            .insert_state(NetworkState::Disconnected)
            .add_plugins(IncomingMessageProcessorPlugin)
            .add_event::<ClientToServerMessage>()
            .add_systems(
                PreUpdate,
                (
                    check_for_connection_updates.run_if(in_state(NetworkState::Connecting)),
                    check_for_connection_updates.run_if(in_state(NetworkState::Connected)),
                ),
            )
            .add_systems(
                PostUpdate,
                (
                    keep_alive.run_if(not(on_event::<ClientToServerMessage>())),
                    event_processor.run_if(on_event::<ClientToServerMessage>()),
                )
                    .chain()
                    .run_if(in_state(NetworkState::Connected)),
            );
    }
}

fn keep_alive(
    time: Res<Time>,
    mut timer: ResMut<KeepAliveTimer>,
    mut event_writer: EventWriter<ClientToServerMessage>,
) {
    if timer.timer.tick(time.delta()).just_finished() {
        event_writer.send(ClientToServerMessage::KeepAlive);
    }
}

#[derive(Resource)]
pub struct KeepAliveTimer {
    timer: Timer,
}

impl Default for KeepAliveTimer {
    fn default() -> Self {
        KeepAliveTimer {
            timer: Timer::new(NETWORK_IDLE_TIMEOUT / 3, TimerMode::Repeating),
        }
    }
}

pub enum ServerConnectionUpdate {
    ConnectionCreated(ServerConnection),
    ConnectionDropped,
}

#[derive(Resource)]
pub struct ServerConnection {
    pub message_receiver: mpsc::UnboundedReceiver<Bytes>,
    pub message_sender: mpsc::UnboundedSender<Bytes>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum NetworkState {
    #[default]
    Disconnected,
    Connecting,
    Connected,
}

fn check_for_connection_updates(
    mut commands: Commands,
    mut network: ResMut<Network>,
    mut next_network_state: ResMut<NextState<NetworkState>>,
) {
    if let Ok(update) = network.connection_receiver.try_recv() {
        match update {
            ServerConnectionUpdate::ConnectionCreated(connection) => {
                commands.insert_resource(KeepAliveTimer::default());
                commands.insert_resource(connection);
                next_network_state.set(NetworkState::Connected);
                info!("Connection Resource has been created.")
            }
            ServerConnectionUpdate::ConnectionDropped => {
                commands.remove_resource::<KeepAliveTimer>();
                commands.remove_resource::<ServerConnection>();
                next_network_state.set(NetworkState::Disconnected);
                info!("Connection has been dropped.")
            }
        }
    }
}

fn event_processor(
    mut events: EventReader<ClientToServerMessage>,
    connection: ResMut<ServerConnection>,
    mut keep_alive_timer: ResMut<KeepAliveTimer>,
) {
    for event in events.read() {
        match event.serialize() {
            Ok(bytes) => {
                let _ = connection.message_sender.send(bytes);
                keep_alive_timer.timer.reset();
            }
            Err(e) => {
                error!(
                    "Failed to serialize NetworkMessage {:?}, Error: {:?}",
                    event, e
                )
            }
        }
    }
}
