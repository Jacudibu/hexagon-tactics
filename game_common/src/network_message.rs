use bevy::prelude::Event;
use hexx::Hex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum NetworkMessage {
    StartGame,
    LoadMap(LoadMap),
    MoveUnit(MoveUnit),
    DebugMessage(DebugMessage),
}

#[derive(Event, Serialize, Deserialize, PartialEq, Debug)]
pub struct LoadMap {
    // TODO: Either send some kind of map identifier or just the entire GameMap struct
    pub path: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct MoveUnit {
    pub to: Hex,
    pub path: Vec<Hex>,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct DebugMessage {
    pub message: String,
}

impl NetworkMessage {
    pub fn serialize(&self) -> bincode::Result<Vec<u8>> {
        bincode::serialize(self)
    }

    pub fn deserialize(bytes: &Vec<u8>) -> bincode::Result<Self> {
        bincode::deserialize(bytes)
    }
}
