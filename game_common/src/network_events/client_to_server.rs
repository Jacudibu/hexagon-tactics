use crate::network_events::NetworkMessage;
use bevy::prelude::Event;
use serde::{Deserialize, Serialize};

#[derive(Event, Serialize, Deserialize, PartialEq, Debug)]
pub enum ClientToServerMessage {
    StartGame,
    FinishedLoading,
}

impl NetworkMessage for ClientToServerMessage {}
