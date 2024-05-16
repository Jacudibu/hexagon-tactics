use crate::networking::network_plugin::{ServerConnection, ServerConnectionUpdate};
use bevy::log::{error, warn};
use bevy::prelude::Resource;
use game_common::network_events::NETWORK_IDLE_TIMEOUT;
use game_common::network_helpers;
use tokio::io::AsyncReadExt;
use tokio::runtime::{Handle, Runtime};
use tokio::sync::mpsc;
use wtransport::{ClientConfig, Endpoint};

#[derive(Resource)]
pub struct Network {
    _tokio_runtime: Runtime,
    tokio_handle: Handle,

    pub(in crate::networking) connection_receiver: mpsc::UnboundedReceiver<ServerConnectionUpdate>,
    connection_sender: mpsc::UnboundedSender<ServerConnectionUpdate>,
}

impl Default for Network {
    fn default() -> Self {
        let tokio_runtime = Runtime::new().unwrap();
        let tokio_handle = tokio_runtime.handle().clone();
        let (tx, rx) = mpsc::unbounded_channel();
        Network {
            _tokio_runtime: tokio_runtime,
            tokio_handle,
            connection_sender: tx,
            connection_receiver: rx,
        }
    }
}

impl Network {
    pub fn connect(&mut self) {
        let _guard = self.tokio_handle.enter();

        let connection_tx = self.connection_sender.clone();

        tokio::spawn(async move {
            let config = ClientConfig::builder()
                .with_bind_default()
                .with_no_cert_validation()
                .keep_alive_interval(Some(NETWORK_IDLE_TIMEOUT / 3))
                .max_idle_timeout(Some(NETWORK_IDLE_TIMEOUT))
                .unwrap()
                .build();

            match Endpoint::client(config)
                .unwrap()
                .connect("https://[::1]:4433")
                .await
            {
                Ok(connection) => {
                    let mut buffer = network_helpers::create_buffer();
                    match connection.open_bi().await.unwrap().await {
                        Ok((mut send_stream, mut receive_stream)) => {
                            let (tx_rx, rx_rx) = mpsc::unbounded_channel();
                            let (tx_tx, mut rx_tx) = mpsc::unbounded_channel();
                            let connection = ServerConnection {
                                message_sender: tx_tx,
                                message_receiver: rx_rx,
                            };
                            if let Err(e) = connection_tx
                                .send(ServerConnectionUpdate::ConnectionCreated(connection))
                            {
                                error!("Internal error while persisting connection: {:?}", e);
                                let _ =
                                    connection_tx.send(ServerConnectionUpdate::ConnectionDropped);
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
                                                error!("Bytes was 0!");
                                                let _ = connection_tx.send(ServerConnectionUpdate::ConnectionDropped);
                                                break;
                                            }
                                            let _ = tx_rx.send(buffer.split().freeze());
                                            network_helpers::reclaim_buffer_capacity_if_necessary(&mut buffer);
                                        }
                                        Err(e) => {
                                            error!("Error when receiving data from server: {:?}", e);
                                            let _ = connection_tx.send(ServerConnectionUpdate::ConnectionDropped);
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                        Err(e) => {
                            error!("Error while opening stream: {:?}", e);
                            let _ = connection_tx.send(ServerConnectionUpdate::ConnectionDropped);
                        }
                    }
                }
                Err(e) => {
                    error!("Error while connecting: {:?}", e);
                    let _ = connection_tx.send(ServerConnectionUpdate::ConnectionDropped);
                }
            };
        });
    }

    pub fn disconnect(&mut self) {
        // TODO
        warn!("Disconnecting isn't yet implemented. You are forever trapped here! :^)")
    }
}
