pub mod client_to_server;
pub mod server_to_client;

use hexx::Hex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct MoveUnit {
    pub to: Hex,
    pub path: Vec<Hex>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DebugMessage {
    pub message: String,
}

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
