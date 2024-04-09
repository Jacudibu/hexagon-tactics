pub mod client_to_server;
pub mod server_to_client;

use bevy::prelude::error;
use bincode::config::Configuration;
use bincode::error::EncodeError;
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::time::Duration;

pub const NETWORK_IDLE_TIMEOUT: Duration = Duration::new(30, 0);

pub trait NetworkMessage: DeserializeOwned + Serialize {
    fn serialize(&self) -> Result<Vec<u8>, EncodeError>;
    fn deserialize(bytes: &[u8]) -> Result<Vec<Self>, ()>;
}

impl<T> NetworkMessage for T
where
    T: DeserializeOwned + Serialize,
{
    fn serialize(&self) -> Result<Vec<u8>, EncodeError> {
        let config = bincode::config::standard();
        bincode::serde::encode_to_vec(self, config)
    }

    fn deserialize(bytes: &[u8]) -> Result<Vec<T>, ()> {
        let config = bincode::config::standard();

        let mut result = Vec::new();
        let mut byte_slice = bytes;
        while byte_slice.len() > 0 {
            match bincode::serde::decode_from_slice::<T, Configuration>(byte_slice, config) {
                Ok((message, read_bytes)) => {
                    result.push(message);
                    byte_slice = &byte_slice[read_bytes..];
                }
                Err(e) => {
                    // Invalid Data or buffer was not big enough
                    error!("Was unable to decode message! {:?}", e)
                }
            }
        }

        Ok(result)
    }
}
