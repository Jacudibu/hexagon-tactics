use crate::network_events::NetworkMessage;
use bevy::prelude::Event;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ServerToClientMessage {
    LoadMap(LoadMap),
}

impl NetworkMessage for ServerToClientMessage {}

#[derive(Event, Serialize, Deserialize, PartialEq, Debug)]
pub struct LoadMap {
    // TODO: Either send some kind of map identifier or just the entire GameMap struct
    pub path: String,
}
