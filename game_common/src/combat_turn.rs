use crate::player::PlayerId;
use crate::unit::{Unit, UnitId};

#[derive(Debug, PartialEq, Eq)]
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
    pub fn start_unit_turn(unit: &Unit) -> CombatTurn {
        CombatTurn::UnitTurn(UnitTurn::start(unit))
    }

    /// panics if self is not CombatTurn::UnitTurn
    #[must_use]
    pub fn as_unit_turn(&self) -> &UnitTurn {
        if let CombatTurn::UnitTurn(unit_turn) = self {
            return unit_turn;
        } else {
            panic!("{:?} is not a UnitTurn!", self);
        }
    }

    /// panics if self is not CombatTurn::UnitTurn
    #[must_use]
    pub fn as_unit_turn_mut(&mut self) -> &mut UnitTurn {
        if let CombatTurn::UnitTurn(unit_turn) = self {
            return unit_turn;
        } else {
            panic!("{:?} is not a UnitTurn!", self);
        }
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
}

impl UnitTurn {
    #[must_use]
    fn start(unit: &Unit) -> UnitTurn {
        UnitTurn {
            unit_id: unit.id,
            remaining_movement: unit.stats_after_buffs.movement,
        }
    }
}
