use crate::player::PlayerId;
use crate::unit::{Unit, UnitId};
use bevy::prelude::Event;
use hexx::Hex;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub enum ServerToClientMessage {
    LoadMap(StartGameAndLoadMap),
    PlayerIsReady(PlayerIsReady),
    AddUnitToPlayer(AddUnitToPlayer),
    PlayerTurnToPlaceUnit(PlayerTurnToPlaceUnit),
    PlaceUnit(PlaceUnit),
    StartUnitTurn(StartUnitTurn),
    MoveUnit(MoveUnit),

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

#[derive(Event, Serialize, Deserialize, PartialEq, Debug)]
pub struct PlayerTurnToPlaceUnit {
    pub player: PlayerId,
}

#[derive(Event, Serialize, Deserialize, PartialEq, Debug)]
pub struct PlaceUnit {
    pub unit_id: UnitId,
    pub hex: Hex,
}

#[derive(Event, Serialize, Deserialize, PartialEq, Debug)]
pub struct StartUnitTurn {
    pub unit_id: UnitId,
}

#[derive(Event, Serialize, Deserialize, PartialEq, Debug)]
pub struct MoveUnit {
    pub path: Vec<Hex>,
}
