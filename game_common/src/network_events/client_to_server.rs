use crate::units::UnitId;
use bevy::prelude::Event;
use hexx::Hex;
use serde::{Deserialize, Serialize};

#[derive(Event, Serialize, Deserialize, PartialEq, Debug)]
pub enum ClientToServerMessage {
    StartGame,
    FinishedLoading,

    PlaceUnit(PlaceUnit),
}

#[derive(Event, Serialize, Deserialize, PartialEq, Debug)]
pub struct PlaceUnit {
    pub unit_id: UnitId,
    pub hex: Hex,
}
