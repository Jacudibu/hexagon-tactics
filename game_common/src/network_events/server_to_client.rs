use crate::network_events::NetworkMessage;
use crate::units::PlayerId;
use bevy::prelude::Event;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ServerToClientMessage {
    LoadMap(StartGameAndLoadMap),
    PlayerIsReady(PlayerIsReady),
    ErrorWhenProcessingMessage(ErrorWhenProcessingMessage),
}

impl NetworkMessage for ServerToClientMessage {}

#[derive(Event, Serialize, Deserialize, PartialEq, Debug)]
pub struct StartGameAndLoadMap {
    // TODO: Either send some kind of map identifier or just the entire GameMap struct
    pub path: String,
}

#[derive(Event, Serialize, Deserialize, PartialEq, Debug)]
pub struct PlayerIsReady {
    pub player_id: PlayerId,
}

#[derive(Event, Serialize, Deserialize, PartialEq, Debug)]
pub struct ErrorWhenProcessingMessage {
    pub message: String,
}
