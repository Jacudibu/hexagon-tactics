use crate::message_processor;
use crate::message_processor::ServerToClientMessageVariant;
use crate::state::SharedState;
use bytes::{Bytes, BytesMut};
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::NetworkMessage;
use std::error::Error;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::sync::{mpsc, Mutex};
use tracing::{debug, error, info, warn};
use wtransport::endpoint::IncomingSession;

pub type ConnectionId = usize;

struct ConnectedClient {
    pub id: ConnectionId,
    pub receiver: mpsc::UnboundedReceiver<Bytes>,
}

pub async fn handle_connection(incoming_session: IncomingSession, state: Arc<Mutex<SharedState>>) {
    let result = handle_connection_impl(incoming_session, state).await;
    error!("{:?}", result);
}

async fn handle_connection_impl(
    incoming_session: IncomingSession,
    state: Arc<Mutex<SharedState>>,
) -> Result<(), Box<dyn Error>> {
    let mut buffer = BytesMut::with_capacity(1024);

    let session_request = incoming_session.await?;

    info!(
        "New session: Authority: '{}', Path: '{}'",
        session_request.authority(),
        session_request.path()
    );

    let connection = session_request.accept().await?;
    let connection_id = connection.stable_id();

    let mut client = {
        let (client_sender, client_receiver) = mpsc::unbounded_channel();
        let mut state = state.lock().await;
        state.connections.insert(connection_id, client_sender);
        state.add_player_and_notify(connection_id);
        ConnectedClient {
            id: connection_id,
            receiver: client_receiver,
        }
    };

    let (mut send_stream, mut receive_stream) = connection.accept_bi().await?;

    loop {
        // TODO: Figure out if this is cancellation-safe
        tokio::select! {
            Some(msg) = client.receiver.recv() => {
                send_stream.write_all(&msg).await?;
            }
            result = receive_stream.read_buf(&mut buffer) => match result {
                Ok(bytes) => {
                    if bytes == 0 {
                        warn!("Bytes was 0!");
                        break;
                    }
                    process_message_from_client(Arc::clone(&state), client.id, buffer.split().freeze()).await;
                }
                Err(e) => {
                    error!(
                        "an error occurred while processing messages for {}; error = {:?}",
                        client.id,
                        e
                    );
                    break;
                }
            }
        }
    }

    state.lock().await.connections.remove(&client.id);
    info!("Connection {} has been removed.", client.id);
    Ok(())
}

async fn process_message_from_client(
    state: Arc<Mutex<SharedState>>,
    connection_id: ConnectionId,
    bytes: Bytes,
) {
    match ClientToServerMessage::deserialize(&bytes.to_vec()) {
        Ok(messages) => {
            let mut state = state.lock().await;
            debug!(
                "Received {} bytes from {}: {:?}",
                bytes.len(),
                connection_id,
                messages
            );
            for message in messages {
                match message_processor::process_message(&mut state, message) {
                    Ok(outgoing_messages) => {
                        for message in outgoing_messages {
                            debug!("Sending {:?}", message);
                            match message {
                                ServerToClientMessageVariant::SendToSender(message) => {
                                    state.send_to(&connection_id, message);
                                }
                                ServerToClientMessageVariant::SendToEveryoneExceptSender(
                                    message,
                                ) => {
                                    state.send_to_everyone_except_one(&connection_id, message);
                                }
                                ServerToClientMessageVariant::Broadcast(message) => {
                                    state.broadcast(message);
                                }
                            }
                        }
                    }
                    Err(error_message) => {
                        state.send_to(&connection_id, error_message);
                    }
                }
            }
        }
        Err(e) => {
            error!(
                "Was unable to deserialize message from bytes! Error: {:?}",
                e
            )
        }
    }
}
