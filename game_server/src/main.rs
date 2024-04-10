use crate::message_processor::ServerToClientMessageVariant;
use crate::state::ClientId;
use bytes::{Bytes, BytesMut};
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::{NetworkMessage, NETWORK_IDLE_TIMEOUT};
use state::{ConnectedClient, SharedState};
use std::error::Error;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::sync::{mpsc, Mutex};
use tracing::{debug, error, info, info_span, warn, Instrument, Level};
use tracing_subscriber::EnvFilter;
use wtransport::endpoint::IncomingSession;
use wtransport::{Endpoint, Identity, ServerConfig};

mod message_processor;
mod state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env(), //    .add_directive("tokio=trace".parse()?)
        )
        //.with_span_events(FmtSpan::FULL)
        .with_max_level(Level::DEBUG)
        .init();

    let port = 4433;
    let config = ServerConfig::builder()
        .with_bind_default(port)
        .with_identity(&Identity::self_signed(["localhost"]))
        .max_idle_timeout(Some(NETWORK_IDLE_TIMEOUT))
        .unwrap()
        .build();

    info!("Server running on port {}", port);

    let server = Endpoint::server(config)?;

    let state = Arc::new(Mutex::new(SharedState::default()));

    for id in 1.. {
        let incoming_session = server.accept().await;
        let state = Arc::clone(&state);

        tokio::spawn(
            handle_connection(incoming_session, state, id).instrument(info_span!("Connection", id)),
        );
    }

    Ok(())
}

async fn handle_connection(
    incoming_session: IncomingSession,
    state: Arc<Mutex<SharedState>>,
    id: ClientId,
) {
    let result = handle_connection_impl(incoming_session, state, id).await;
    error!("{:?}", result);
}

async fn handle_connection_impl(
    incoming_session: IncomingSession,
    state: Arc<Mutex<SharedState>>,
    id: ClientId,
) -> Result<(), Box<dyn Error>> {
    let mut buffer = BytesMut::with_capacity(1024);

    let session_request = incoming_session.await?;

    info!(
        "New session: Authority: '{}', Path: '{}'",
        session_request.authority(),
        session_request.path()
    );

    let connection = session_request.accept().await?;

    let mut client = {
        let (client_sender, client_receiver) = mpsc::unbounded_channel();
        state.lock().await.connections.insert(id, client_sender);
        ConnectedClient {
            id,
            receiver: client_receiver,
        }
    };

    let (mut send_stream, mut receive_stream) = connection.accept_bi().await?;

    loop {
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
    client_id: ClientId,
    bytes: Bytes,
) {
    match ClientToServerMessage::deserialize(&bytes.to_vec()) {
        Ok(messages) => {
            let mut state = state.lock().await;
            debug!(
                "Received {} bytes from {}: {:?}",
                bytes.len(),
                client_id,
                messages
            );
            for message in messages {
                match message_processor::process_message(&mut state, message) {
                    Ok(outgoing_messages) => {
                        for message in outgoing_messages {
                            debug!("Sending {:?}", message);
                            match message {
                                ServerToClientMessageVariant::SendToSender(message) => {
                                    state.send_to(&client_id, message);
                                }
                                ServerToClientMessageVariant::SendToEveryoneExceptSender(
                                    message,
                                ) => {
                                    state.send_to_everyone_except_one(&client_id, message);
                                }
                                ServerToClientMessageVariant::Broadcast(message) => {
                                    state.broadcast(message);
                                }
                            }
                        }
                    }
                    Err(error_message) => {
                        state.send_to(&client_id, error_message);
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
