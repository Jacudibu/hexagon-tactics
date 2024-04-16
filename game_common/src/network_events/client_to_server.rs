use crate::unit::UnitId;
use bevy::prelude::Event;
use hexx::Hex;
use serde::{Deserialize, Serialize};

#[derive(Event, Serialize, Deserialize, PartialEq, Debug)]
pub enum ClientToServerMessage {
    StartGame,
    FinishedLoading,
    EndTurn,

    PlaceUnit(PlaceUnit),
    MoveUnit(MoveUnit),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PlaceUnit {
    pub unit_id: UnitId,
    pub hex: Hex,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct MoveUnit {
    pub path: Vec<Hex>,
}
