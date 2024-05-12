use crate::combat_unit::UnitId;
use crate::game_data::skill::SkillId;
use bevy::prelude::Event;
use hexx::Hex;
use serde::{Deserialize, Serialize};

#[derive(Event, Serialize, Deserialize, PartialEq, Debug)]
pub enum ClientToServerMessage {
    // Lobby
    StartGame,

    // Combat
    FinishedLoading,
    EndTurn,
    PlaceUnit(PlaceUnit),
    MoveUnit(MoveUnit),
    UseSkill(UseSkill),

    // Events
    PickUnit(PickUnit),
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct PickUnit {
    pub unit_id: UnitId,
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

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct UseSkill {
    pub id: SkillId,
    pub target_coordinates: Hex,
}
