use crate::combat_unit::{CombatUnit, UnitId};
use crate::player::PlayerId;
use enum_as_inner::EnumAsInner;

#[derive(Debug, PartialEq, Eq, EnumAsInner)]
pub enum CombatTurn {
    Undefined,
    PlaceUnit(PlaceUnit),
    UnitTurn(UnitTurn),
}

impl CombatTurn {
    #[must_use]
    pub fn place_unit(player_id: PlayerId) -> CombatTurn {
        CombatTurn::PlaceUnit(PlaceUnit { player_id })
    }

    #[must_use]
    pub fn start_unit_turn(unit: &CombatUnit) -> CombatTurn {
        CombatTurn::UnitTurn(UnitTurn::start(unit))
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct PlaceUnit {
    pub player_id: PlayerId,
}

#[derive(Debug, PartialEq, Eq)]
pub struct UnitTurn {
    pub unit_id: UnitId,
    pub remaining_movement: u8,
    pub remaining_actions: u8,
}

impl UnitTurn {
    #[must_use]
    fn start(unit: &CombatUnit) -> UnitTurn {
        UnitTurn {
            unit_id: unit.id,
            remaining_movement: unit.stats_after_buffs.movement,
            remaining_actions: 1,
        }
    }
}
