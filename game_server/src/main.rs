use crate::message_processor::ServerToClientMessageVariant;
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::{NetworkMessage, NETWORK_IDLE_TIMEOUT};
use state::{ConnectedClient, SharedState};
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, error, info, info_span, Instrument, Level};
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

    for id in 0.. {
        let incoming_session = server.accept().await;
        let state = Arc::clone(&state);

        tokio::spawn(
            handle_connection(incoming_session, state).instrument(info_span!("Connection", id)),
        );
    }

    Ok(())
}

async fn handle_connection(incoming_session: IncomingSession, state: Arc<Mutex<SharedState>>) {
    let result = handle_connection_impl(incoming_session, state).await;
    error!("{:?}", result);
}

async fn handle_connection_impl(
    incoming_session: IncomingSession,
    state: Arc<Mutex<SharedState>>,
) -> Result<(), Box<dyn Error>> {
    let mut buffer = vec![0; 65536].into_boxed_slice();

    info!("Waiting for session request...");

    let session_request = incoming_session.await?;

    info!(
        "New session: Authority: '{}', Path: '{}'",
        session_request.authority(),
        session_request.path()
    );

    let connection = session_request.accept().await?;

    info!("Waiting for data from client...");

    let mut client = ConnectedClient::new(state.clone(), &connection).await?;

    let (mut send_stream, mut receive_stream) = connection.accept_bi().await?;
    info!("Accepted BI stream");

    loop {
        tokio::select! {
            Some(msg) = client.rx.recv() => {
                send_stream.write_all(&msg).await?;
            }
            result = receive_stream.read(&mut buffer) => match result {
                Ok(Some(bytes)) => {
                    //send_stream.write_all(b"ACK").await?;
                    process_message_from_client(Arc::clone(&state), client.id, buffer[..bytes].to_vec()).await;
                }
                Err(e) => {
                    error!(
                        "an error occurred while processing messages for {}; error = {:?}",
                        client.id,
                        e
                    );
                }
                // The stream has been exhausted.
                Ok(None) => {break;}
            }
        }
    }

    Ok(())
}

async fn process_message_from_client(
    state: Arc<Mutex<SharedState>>,
    sender: usize,
    bytes: Vec<u8>,
) {
    match ClientToServerMessage::deserialize(&bytes.to_vec()) {
        Ok(messages) => {
            let mut state = state.lock().await;
            debug!(
                "Received {} bytes from {}: {:?}",
                bytes.len(),
                sender,
                messages
            );
            for message in messages {
                match message_processor::process_message(&mut state, message) {
                    Ok(outgoing_messages) => {
                        for message in outgoing_messages {
                            debug!("Sending {:?}", message);
                            match message {
                                ServerToClientMessageVariant::SendToSender(message) => {
                                    state.send_to(&sender, message);
                                }
                                ServerToClientMessageVariant::SendToEveryoneExceptSender(
                                    message,
                                ) => {
                                    state.send_to_everyone_except_one(&sender, message);
                                }
                                ServerToClientMessageVariant::Broadcast(message) => {
                                    state.broadcast(message);
                                }
                            }
                        }
                    }
                    Err(error_message) => {
                        state.send_to(&sender, error_message);
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
