use crate::network_events::NetworkMessage;
use bevy::prelude::Event;
use serde::{Deserialize, Serialize};

#[derive(Event, Serialize, Deserialize, PartialEq, Debug)]
pub enum ClientToServerMessage {
    Heartbeat,
    StartGame,
    FinishedLoading,
}

impl NetworkMessage<'_> for ClientToServerMessage {}
