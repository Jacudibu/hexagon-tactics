use bevy::prelude::{
    error, in_state, info, trace, App, Commands, IntoSystemConfigs, Local, NextState, OnEnter,
    Plugin, PostUpdate, PreUpdate, Res, ResMut, Resource, StateTransition, States, Time,
};
use futures::SinkExt;
use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf};
use tokio::net::TcpStream;
use tokio::runtime::{Handle, Runtime};
use tokio::sync::mpsc;
use tokio_stream::StreamExt;
use tokio_util::codec::{Framed, LinesCodec};

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
                Ok(mut stream) => {
                    info!("created stream");

                    let result = stream.write_all(b"Hello world").await;
                    info!("wrote to stream; success={:?}", result.is_ok());

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

                    let mut lines = Framed::new(stream, LinesCodec::new());
                    loop {
                        tokio::select! {
                            // Sending
                            Some(msg) = rx_tx.recv() => {
                                let msg = String::from_utf8(msg).expect("");
                                let _ = lines.send(&msg).await;
                            }
                            // Receiving
                            result = lines.next() => match result {
                                Some(Ok(value)) => {
                                    info!("Received {}", value);
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

fn receive_updates(connection: Res<ServerConnection>) {
    // TODO: Actually Receive Updates
    // TODO: Create Events for updates
}

#[derive(Default)]
struct DebugLocal {
    i: i32,
}

fn send_updates(
    time: Res<Time>,
    mut connection: ResMut<ServerConnection>,
    mut local: Local<DebugLocal>,
) {
    // TODO: Actually send Updates
    let value = local.i;

    if value > time.elapsed_seconds() as i32 {
        return;
    }

    trace!("sending!");

    local.i = time.elapsed_seconds() as i32 + 1;
    let data = format!("{}", time.elapsed_seconds()).into_bytes();
    let bytes = connection.message_tx.send(data);
}
