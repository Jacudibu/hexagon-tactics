use crate::units::{PlayerId, Unit};
use bevy::prelude::Event;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ServerToClientMessage {
    LoadMap(StartGameAndLoadMap),
    PlayerIsReady(PlayerIsReady),
    AddUnitToPlayer(AddUnitToPlayer),

    ErrorWhenProcessingMessage(ErrorWhenProcessingMessage),
}

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

#[derive(Event, Serialize, Deserialize, PartialEq, Debug)]
pub struct AddUnitToPlayer {
    pub unit: Unit,
}
