use std::collections::HashMap;
use std::error::Error;
use std::io;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::io::AsyncReadExt;
use tokio::net::{TcpListener, TcpStream};
use tokio::sync::{mpsc, Mutex};
use tokio_util::codec::{Framed, LinesCodec};
use tokio_stream::StreamExt;
use futures::SinkExt;
use tracing_subscriber::EnvFilter;
use tracing_subscriber::fmt::format::FmtSpan;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env()
        //    .add_directive("tokio=trace".parse()?)
        )
        //.with_span_events(FmtSpan::FULL)
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let addr = "127.0.0.1:1337";
    let listener = TcpListener::bind(addr).await.unwrap();

    tracing::info!("server running on {}", addr);

    let state = Arc::new(Mutex::new(SharedState::default()));

    loop {
        let (stream, addr) = listener.accept().await?;
        let state = Arc::clone(&state);

        tokio::spawn(async move {
            tracing::debug!("accepted connection");
            if let Err(e) = process_incoming_connection(state, stream, addr).await {
                tracing::info!("an error occurred; error = {:?}", e);
            }
        });
    }

}

#[derive(Default)]
struct SharedState {
    connections: HashMap<SocketAddr, mpsc::UnboundedSender<String>>
}

struct ConnectedClient {
    lines: Framed<TcpStream, LinesCodec>,
    rx: mpsc::UnboundedReceiver<String>,
}

impl ConnectedClient {
    async fn new(
        state: Arc<Mutex<SharedState>>,
        lines: Framed<TcpStream, LinesCodec>,
    ) -> io::Result<ConnectedClient> {
        let addr = lines.get_ref().peer_addr()?;
        let (tx, rx) = mpsc::unbounded_channel();
        state.lock().await.connections.insert(addr, tx);
        Ok(ConnectedClient { lines, rx })
    }
}


async fn process_incoming_connection(state: Arc<Mutex<SharedState>>, mut stream: TcpStream, addr: SocketAddr) -> Result<(), Box<dyn Error>> {
    let mut lines = Framed::new(stream, LinesCodec::new());
    let mut client = ConnectedClient::new(state.clone(), lines).await?;

    loop {
        tokio::select! {
            // A message was scheduled to be sent to this client
            Some(msg) = client.rx.recv() => {
                client.lines.send(&msg).await?;
            }
            result = client.lines.next() => match result {
                // A message was received from the client
                Some(Ok(msg)) => {
                    let mut state = state.lock().await;
                    let msg = format!("{}: {}", addr, msg);

                    tracing::info!(msg);
                }
                // An error occurred.
                Some(Err(e)) => {
                    tracing::error!(
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
        tracing::info!("{}", msg);
    }

    Ok(())
}
