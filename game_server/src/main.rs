use futures::SinkExt;
use game_common::game_map::GameMap;
use game_common::game_state::GameState;
use game_common::network_message::{DebugMessage, LoadMap, NetworkMessage};
use game_common::TEST_MAP_NAME;
use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use tokio_stream::StreamExt;
use tokio_util::bytes::BytesMut;
use tokio_util::codec::{BytesCodec, Framed};
use tracing::{debug, error, info, Level};
use tracing_subscriber::EnvFilter;

mod message_processor;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env(), //    .add_directive("tokio=trace".parse()?)
        )
        //.with_span_events(FmtSpan::FULL)
        .with_max_level(Level::DEBUG)
        .init();

    let addr = "127.0.0.1:1337";
    let listener = TcpListener::bind(addr).await.unwrap();

    info!("server running on {}", addr);

    let state = Arc::new(Mutex::new(SharedState::default()));

    loop {
        let (stream, addr) = listener.accept().await?;
        let state = Arc::clone(&state);

        tokio::spawn(async move {
            debug!("accepted connection");
            if let Err(e) = process_incoming_connection(state, stream, addr).await {
                info!("an error occurred; error = {:?}", e);
            }
        });
    }
}

#[derive(Default)]
struct SharedState {
    connections: HashMap<SocketAddr, mpsc::UnboundedSender<Vec<u8>>>,
    game_states: Vec<GameState>,
}

impl SharedState {
    async fn broadcast(&mut self, message: &NetworkMessage) {
        match message.serialize() {
            Ok(bytes) => {
                for (_, tx) in self.connections.iter_mut() {
                    // Wonder if there's some way of doing this without cloning?
                    let _ = tx.send(bytes.clone());
                }
            }
            Err(e) => {
                error!(
                    "Error when trying to serialize NetworkMessage {:?} - Error: {:?}",
                    message, e
                );
            }
        }
    }
}

struct ConnectedClient {
    frame: Framed<TcpStream, BytesCodec>,
    rx: mpsc::UnboundedReceiver<Vec<u8>>,
}

impl ConnectedClient {
    async fn new(
        state: Arc<Mutex<SharedState>>,
        frame: Framed<TcpStream, BytesCodec>,
    ) -> io::Result<ConnectedClient> {
        let addr = frame.get_ref().peer_addr()?;
        let (tx, rx) = mpsc::unbounded_channel();
        state.lock().await.connections.insert(addr, tx);
        Ok(ConnectedClient { frame, rx })
    }
}

async fn process_incoming_connection(
    state: Arc<Mutex<SharedState>>,
    stream: TcpStream,
    addr: SocketAddr,
) -> Result<(), Box<dyn Error>> {
    let frame = Framed::new(stream, BytesCodec::new());
    let mut client = ConnectedClient::new(state.clone(), frame).await?;

    loop {
        tokio::select! {
            // Outgoing
            Some(msg) = client.rx.recv() => {
                let bytes = BytesMut::from_iter(msg);
                client.frame.send(bytes).await?;
            }
            // Incoming
            result = client.frame.next() => match result {
                Some(Ok(msg)) => {
                    process_message_from_client(Arc::clone(&state), addr, msg).await;
                }
                Some(Err(e)) => {
                    error!(
                        "an error occurred while processing messages for {}; error = {:?}",
                        addr,
                        e
                    );
                }
                // The stream has been exhausted.
                None => break,
            },
        }
    }

    // If this section is reached it means that the client was disconnected!
    {
        let mut state = state.lock().await;
        state.connections.remove(&addr);

        let msg = format!("{} disconnected.", addr);
        info!("{}", msg);
    }

    Ok(())
}

async fn process_message_from_client(
    state: Arc<Mutex<SharedState>>,
    sender: SocketAddr,
    bytes: BytesMut,
) {
    match NetworkMessage::deserialize(&bytes.to_vec()) {
        Ok(message) => {
            let mut state = state.lock().await;
            info!("Processing message from {}: {:?}", sender, message);

            // TODO: Execute fancy game logic, broadcast result
            match message_processor::process_message(&mut state, message) {
                Ok(resulting_message) => {
                    state.broadcast(&resulting_message).await;
                }
                Err(_) => {
                    // TODO: Send error to client.
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
