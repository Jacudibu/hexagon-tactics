use bevy::prelude::{
    debug, error, in_state, info, not, on_event, warn, App, Commands, EventReader, EventWriter,
    IntoSystemConfigs, NextState, Plugin, PostUpdate, PreUpdate, Res, ResMut, Resource, States,
    Timer, TimerMode,
};
use bevy::time::Time;
use bytes::{Bytes, BytesMut};
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::server_to_client::ServerToClientMessage;
use game_common::network_events::{server_to_client, NetworkMessage, NETWORK_IDLE_TIMEOUT};
use tokio::io::AsyncReadExt;
use tokio::runtime::{Handle, Runtime};
use tokio::sync::mpsc;
use wtransport::{ClientConfig, Endpoint};

pub struct NetworkPlugin;
impl Plugin for NetworkPlugin {
    fn build(&self, app: &mut App) {
        let tokio_runtime = Runtime::new().unwrap();
        let tokio_handle = tokio_runtime.handle().clone();
        let (tx, rx) = mpsc::unbounded_channel();
        let network = Network {
            _tokio_runtime: tokio_runtime,
            tokio_handle,
            connection_tx: tx,
            connection_rx: rx,
        };

        app.insert_resource(network)
            .insert_state(NetworkState::Disconnected)
            .add_event::<ClientToServerMessage>()
            .add_event::<server_to_client::StartGameAndLoadMap>()
            .add_event::<server_to_client::PlayerIsReady>()
            .add_event::<server_to_client::AddUnitToPlayer>()
            .add_systems(
                PreUpdate,
                (
                    check_for_connection.run_if(in_state(NetworkState::Connecting)),
                    receive_updates.run_if(in_state(NetworkState::Connected)),
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

#[derive(Resource)]
pub struct Network {
    _tokio_runtime: Runtime,
    tokio_handle: Handle,

    connection_rx: mpsc::UnboundedReceiver<ServerConnection>,
    connection_tx: mpsc::UnboundedSender<ServerConnection>,
}

#[derive(Resource)]
pub struct ServerConnection {
    message_rx: mpsc::UnboundedReceiver<Bytes>,
    message_tx: mpsc::UnboundedSender<Bytes>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
pub enum NetworkState {
    #[default]
    Disconnected,
    Connecting,
    Connected,
}

impl Network {
    pub fn connect(&mut self) {
        let _guard = self.tokio_handle.enter();

        let connection_tx = self.connection_tx.clone();

        tokio::spawn(async move {
            let config = ClientConfig::builder()
                .with_bind_default()
                .with_no_cert_validation()
                .max_idle_timeout(Some(NETWORK_IDLE_TIMEOUT))
                .unwrap()
                .build();

            match Endpoint::client(config)
                .unwrap()
                .connect("https://[::1]:4433")
                .await
            {
                Ok(connection) => {
                    let mut buffer = BytesMut::with_capacity(1024);
                    match connection.open_bi().await.unwrap().await {
                        Ok((mut send_stream, mut receive_stream)) => {
                            let (tx_rx, rx_rx) = mpsc::unbounded_channel();
                            let (tx_tx, mut rx_tx) = mpsc::unbounded_channel();
                            let connection = ServerConnection {
                                message_tx: tx_tx,
                                message_rx: rx_rx,
                            };
                            if let Err(e) = connection_tx.send(connection) {
                                error!("Internal error while persisting connection: {:?}", e);
                                return;
                            }

                            loop {
                                tokio::select! {
                                    Some(bytes) = rx_tx.recv() => {
                                        let _ = send_stream.write_all(&bytes).await;
                                    }
                                    result = receive_stream.read_buf(&mut buffer) => match result {
                                        Ok(bytes) => {
                                            if bytes == 0 {
                                                info!("Bytes was 0!");
                                                break;
                                            }
                                            let _ = tx_rx.send(buffer.split().freeze());
                                        }
                                        Err(e) => {
                                            error!("Error when receiving data from server: {:?}", e);
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            error!("Error while opening stream: {:?}", e);
                        }
                    }
                }
                Err(e) => {
                    error!("Error while connecting: {:?}", e);
                }
            };
        });
    }

    pub fn disconnect(&mut self) {
        // TODO
        warn!("Disconnecting isn't yet implemented. You are forever trapped here! :^)")
    }
}

fn check_for_connection(
    mut commands: Commands,
    mut network: ResMut<Network>,
    mut next_network_state: ResMut<NextState<NetworkState>>,
) {
    if let Ok(connection) = network.connection_rx.try_recv() {
        commands.insert_resource(connection);
        commands.insert_resource(KeepAliveTimer::default());
        next_network_state.set(NetworkState::Connected);
        info!("Connection Resource has been created.")
    }
}

fn receive_updates(
    mut connection: ResMut<ServerConnection>,
    mut load_map_event_from_server: EventWriter<server_to_client::StartGameAndLoadMap>,
    mut player_is_ready: EventWriter<server_to_client::PlayerIsReady>,
    mut add_unit_to_player: EventWriter<server_to_client::AddUnitToPlayer>,
) {
    if let Ok(bytes) = connection.message_rx.try_recv() {
        match ServerToClientMessage::deserialize(&bytes) {
            Ok(messages) => {
                debug!("Received {} bytes: {:?}", bytes.len(), messages);
                for message in messages {
                    match message {
                        ServerToClientMessage::LoadMap(event) => {
                            load_map_event_from_server.send(event);
                        }

                        ServerToClientMessage::PlayerIsReady(event) => {
                            player_is_ready.send(event);
                        }

                        ServerToClientMessage::AddUnitToPlayer(event) => {
                            add_unit_to_player.send(event);
                        }

                        ServerToClientMessage::ErrorWhenProcessingMessage(e) => {
                            error!("Server responded with an error: {:?}", e);
                        }
                    };
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
}

fn event_processor(
    mut events: EventReader<ClientToServerMessage>,
    connection: ResMut<ServerConnection>,
    mut keep_alive_timer: ResMut<KeepAliveTimer>,
) {
    for event in events.read() {
        match event.serialize() {
            Ok(bytes) => {
                let _ = connection.message_tx.send(bytes);
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
