use game_common::network_events::NETWORK_IDLE_TIMEOUT;
use shared_state::SharedState;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{info, Level};
use tracing_subscriber::EnvFilter;
use wtransport::{Endpoint, Identity, ServerConfig};

mod connection_handler;
mod in_game;
mod lobby;
mod message_processor;
mod server_state;
mod shared_state;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    setup_tracing();

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

    for _ in 1.. {
        let incoming_session = server.accept().await;
        let state = Arc::clone(&state);

        tokio::spawn(connection_handler::handle_connection(
            incoming_session,
            state,
        ));
    }

    Ok(())
}

fn setup_tracing() {
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::from_default_env(), //    .add_directive("tokio=trace".parse()?)
        )
        //.with_span_events(FmtSpan::FULL)
        .with_max_level(Level::DEBUG)
        .init();
}
