use hexx::Hex;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub type PlayerId = u8;
pub type UnitId = u8;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Unit {
    pub id: UnitId,
    pub owner: PlayerId,
    pub name: String,
    pub position: Option<Hex>, // TODO: Once we start differentiating between units in combat and "units stored", this should no longer be an option
    pub hp: u32,
    pub mp: u32,
    pub exp: u32,
    pub base_stats: UnitStats,
    pub stats_after_buffs: UnitStats,
    pub turn_resources: TurnResources,
}

impl PartialEq<Self> for Unit {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", self.name, self.id)
    }
}

impl Unit {
    pub fn create_debug_unit(id: UnitId, owner: PlayerId, name: String) -> Self {
        let movement = 4;

        Unit {
            id,
            owner,
            name,
            position: None,
            hp: 10,
            mp: 10,
            exp: 0,
            base_stats: UnitStats {
                movement,
                jump: 3,
                strength: 10,
            },
            stats_after_buffs: UnitStats {
                movement,
                jump: 3,
                strength: 10,
            },
            turn_resources: TurnResources {
                remaining_movement: movement,
            },
        }
    }
}

#[derive(Debug)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", self.name, self.id)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnitStats {
    pub movement: u8,
    pub jump: u8,
    pub strength: u32,
    // ...and other stats later on
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TurnResources {
    pub remaining_movement: u8,
}
