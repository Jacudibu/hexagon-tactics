use bevy::prelude::Event;
use serde::{Deserialize, Serialize};

#[derive(Event, Serialize, Deserialize, PartialEq, Debug)]
pub enum ClientToServerMessage {
    KeepAlive,
    StartGame,
    FinishedLoading,
}
