pub mod client_to_server;
pub mod server_to_client;

use serde::Deserialize;

pub trait NetworkMessage {
    fn serialize(&self) -> bincode::Result<Vec<u8>>
    where
        Self: serde::Serialize,
    {
        bincode::serialize(self)
    }

    fn deserialize<'a>(bytes: &'a Vec<u8>) -> bincode::Result<Self>
    where
        Self: Sized,
        Self: Deserialize<'a>,
    {
        bincode::deserialize(bytes)
    }
}
