use crate::player::{Player, PlayerId, ReadyState};
use crate::unit::{Unit, UnitId};
use bevy::prelude::Event;
use hexx::Hex;
use serde::{Deserialize, Serialize};

use crate::skill::{SkillId, SkillInvocationResult};
#[cfg(feature = "test_helpers")]
use enum_as_inner::EnumAsInner;

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[cfg_attr(feature = "test_helpers", derive(EnumAsInner))]
pub enum ServerToClientMessage {
    YouConnected(YouConnected),
    OtherPlayerConnected(OtherPlayerConnected),

    LoadMap(StartGameAndLoadMap),
    PlayerIsReady(UpdateReadyStateForPlayer),
    AddUnitToPlayerStorage(AddUnitToPlayerStorage),
    PlayerTurnToPlaceUnit(PlayerTurnToPlaceUnit),
    PlaceUnit(PlaceUnit),
    StartUnitTurn(StartUnitTurn),
    MoveUnit(MoveUnit),
    UseSkill(UseSkill),

    ErrorWhenProcessingMessage(ErrorWhenProcessingMessage),
}

#[derive(Event, Serialize, Deserialize, PartialEq, Debug)]
pub struct StartGameAndLoadMap {
    // TODO: Either send some kind of map identifier or just the entire GameMap struct
    pub path: String,
}

#[derive(Event, Serialize, Deserialize, PartialEq, Debug)]
pub struct YouConnected {
    pub player_id: PlayerId,
    pub connected_players: Vec<Player>,
}

#[derive(Event, Serialize, Deserialize, PartialEq, Debug)]
pub struct OtherPlayerConnected {
    pub player: Player,
}

#[derive(Event, Serialize, Deserialize, PartialEq, Debug)]
pub struct UpdateReadyStateForPlayer {
    pub player_id: PlayerId,
    pub ready_state: ReadyState,
}

#[derive(Event, Serialize, Deserialize, PartialEq, Debug)]
pub struct ErrorWhenProcessingMessage {
    pub message: String,
}

#[derive(Event, Serialize, Deserialize, PartialEq, Debug)]
pub struct AddUnitToPlayerStorage {
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

#[derive(Event, Serialize, Deserialize, PartialEq, Debug)]
pub struct UseSkill {
    pub id: SkillId,
    pub result: SkillInvocationResult,
}
