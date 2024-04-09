pub mod client_to_server;
pub mod server_to_client;

use serde::{Deserialize, Serialize};
use std::time::Duration;

pub const NETWORK_IDLE_TIMEOUT: Duration = Duration::new(30, 0);

pub trait NetworkMessage<'a>: Deserialize<'a> + Serialize {
    fn serialize(&self) -> bincode::Result<Vec<u8>> {
        bincode::serialize(self)
    }

    fn deserialize(bytes: &'a Vec<u8>) -> bincode::Result<Self> {
        bincode::deserialize(bytes)
    }
}
