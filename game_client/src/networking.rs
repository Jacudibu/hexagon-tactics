use bevy::input::ButtonInput;
use bevy::prelude::{
    error, in_state, info, App, Commands, IntoSystemConfigs, KeyCode, NextState, Plugin,
    PostUpdate, PreUpdate, Res, ResMut, Resource, States, Time,
};
use futures::SinkExt;
use game_common::network_message::{DebugMessage, NetworkMessage};
use tokio::net::TcpStream;
use tokio::runtime::{Handle, Runtime};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use tokio_util::bytes::BytesMut;
use tokio_util::codec::{BytesCodec, Framed};

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
            .add_systems(
                PreUpdate,
                (
                    check_for_connection.run_if(in_state(NetworkState::Disconnected)),
                    receive_updates.run_if(in_state(NetworkState::Connected)),
                ),
            )
            .add_systems(
                PostUpdate,
                send_updates.run_if(in_state(NetworkState::Connected)),
            );
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
    message_rx: mpsc::UnboundedReceiver<Vec<u8>>,
    message_tx: mpsc::UnboundedSender<Vec<u8>>,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default, States)]
enum NetworkState {
    #[default]
    Disconnected,
    Connected,
}

impl Network {
    pub fn connect(&mut self) {
        let _guard = self.tokio_handle.enter();

        let connection_tx = self.connection_tx.clone();

        tokio::spawn(async move {
            match TcpStream::connect("127.0.0.1:1337").await {
                Ok(stream) => {
                    let (tx_rx, rx_rx) = mpsc::unbounded_channel();
                    let (tx_tx, mut rx_tx) = mpsc::unbounded_channel();
                    let connection = ServerConnection {
                        message_tx: tx_tx,
                        message_rx: rx_rx,
                    };
                    match connection_tx.send(connection) {
                        Ok(_) => {
                            info!("Connection has been sent to main thread.");
                        }
                        Err(e) => {
                            info!("Internal error while persisting connection: {:?}", e);
                        }
                    }

                    let mut frame = Framed::new(stream, BytesCodec::new());
                    loop {
                        tokio::select! {
                            // Sending
                            Some(bytes) = rx_tx.recv() => {
                                let bytes = BytesMut::from_iter(bytes);
                                let _ = frame.send(bytes).await;
                            }
                            // Receiving
                            result = frame.next() => match result {
                                Some(Ok(bytes)) => {
                                    let _ = tx_rx.send(bytes.to_vec());
                                }
                                Some(Err(e)) => {
                                    error!("Error when receiving data from server: {:?}", e)
                                }
                                None => break,
                            }
                        }
                    }
                }
                Err(e) => {
                    info!("Error while connecting: {:?}", e);
                }
            }
        });
    }
}

fn check_for_connection(
    mut commands: Commands,
    mut network: ResMut<Network>,
    mut next_network_state: ResMut<NextState<NetworkState>>,
) {
    if let Ok(connection) = network.connection_rx.try_recv() {
        commands.insert_resource(connection);
        next_network_state.set(NetworkState::Connected);
        info!("Connection Resource has been created.")
    }
}

fn receive_updates(mut connection: ResMut<ServerConnection>) {
    if let Ok(bytes) = connection.message_rx.try_recv() {
        match NetworkMessage::deserialize(&bytes) {
            Ok(message) => {
                info!("Received message {:?}", message);
                // TODO: Create Event to process it properly
            }
            Err(e) => {
                error!(
                    "Failed deserializing NetworkMessage! Error: {:?} Bytes: {:?}",
                    e, bytes
                )
            }
        }
    }
}

fn send_updates(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    connection: ResMut<ServerConnection>,
) {
    // TODO: Actually send Updates instead of triggering them through key presses
    for key in keyboard_input.get_just_pressed() {
        let message = NetworkMessage::DebugMessage(DebugMessage {
            message: format!("{:?} ({})", key, time.elapsed_seconds()),
        });
        match message.serialize() {
            Ok(bytes) => {
                let _ = connection.message_tx.send(bytes);
            }
            Err(e) => {
                error!(
                    "Failed to serialize NetworkMessage {:?}, Error: {:?}",
                    message, e
                )
            }
        }
    }
}
