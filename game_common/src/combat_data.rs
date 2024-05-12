use crate::combat_turn::CombatTurn;
use crate::combat_unit::{CombatUnit, UnitId};
use crate::game_map::GameMap;
use bevy::prelude::{error, Resource};
use bevy::utils::HashMap;
use hexx::Hex;

/// Combat Data shared between client and server. Things in here should always be kept in sync.
#[derive(Debug)]
#[cfg_attr(feature = "ecs", derive(Resource))]
pub struct CombatData {
    pub units: HashMap<UnitId, CombatUnit>,
    pub unit_positions: HashMap<Hex, UnitId>,
    pub current_turn: CombatTurn,
}

impl CombatData {
    pub fn start_unit_turn(&mut self, unit_id: UnitId) {
        let unit = &self.units[&unit_id];
        self.current_turn = CombatTurn::start_unit_turn(unit);
    }

    /// Returns a reference to the currently active unit.
    /// # Panics
    /// In case turn data is invalid or there is no active unit.
    #[must_use]
    pub fn current_turn_unit(&self) -> &CombatUnit {
        &self.units[&self.current_turn.as_unit_turn().unwrap().unit_id]
    }

    #[must_use]
    pub fn can_unit_be_placed_on_tile(&self, hex: &Hex, map: &GameMap) -> bool {
        if self.unit_positions.contains_key(hex) {
            return false;
        }

        let Some(tile) = map.tiles.get(hex) else {
            error!("Invalid tile coordinates: {:?}", hex);
            return false;
        };

        if let Some(fluid) = &tile.fluid {
            if fluid.height > 1.0 {
                return false;
            }
        }

        true
    }

    /// The bigger this value is, the more valuable a high speed stat would be
    const COUNTER_NEEDED_FOR_TURN: u32 = 100;

    #[must_use]
    pub fn get_unit_for_next_turn(&mut self) -> UnitId {
        let mut ready_units = Vec::new();
        let mut highest_counter = Self::COUNTER_NEEDED_FOR_TURN;
        for x in self.units.values() {
            if x.is_dead() {
                continue;
            }

            if x.turn_counter >= highest_counter {
                if x.turn_counter > highest_counter {
                    ready_units.clear();
                    highest_counter = x.turn_counter;
                }
                ready_units.push(TurnOrderElement::from(x));
            }
        }

        while ready_units.is_empty() {
            for x in self.units.values_mut() {
                if x.is_dead() {
                    continue;
                }

                x.turn_counter += x.stats_after_buffs.speed;
                if x.turn_counter >= highest_counter {
                    if x.turn_counter > highest_counter {
                        ready_units.clear();
                        highest_counter = x.turn_counter;
                    }
                    ready_units.push(TurnOrderElement::from(x));
                }
            }
        }

        let unit = if ready_units.len() > 1 {
            ready_units.sort_by(|a, b| b.tiebreaker.cmp(&a.tiebreaker));
            self.units
                .get_mut(&ready_units[1].unit_id)
                .unwrap()
                .turn_tiebreaker = ready_units[0].tiebreaker;
            let unit = self.units.get_mut(&ready_units[0].unit_id).unwrap();
            unit.turn_tiebreaker = ready_units[1].tiebreaker;
            unit
        } else {
            self.units.get_mut(&ready_units[0].unit_id).unwrap()
        };

        unit.turn_counter -= Self::COUNTER_NEEDED_FOR_TURN;
        unit.id
    }
}

struct TurnOrderElement {
    unit_id: UnitId,
    tiebreaker: u32,
}

impl TurnOrderElement {
    #[must_use]
    fn from(unit: &CombatUnit) -> TurnOrderElement {
        TurnOrderElement {
            tiebreaker: unit.turn_tiebreaker,
            unit_id: unit.id,
        }
    }
}

#[cfg(feature = "test_helpers")]
pub mod test_helpers {
    use crate::combat_data::CombatData;
    use crate::combat_turn::CombatTurn;
    use crate::combat_unit::{CombatUnit, UnitId};
    use bevy::utils::HashMap;

    impl CombatData {
        /// Create mock CombatData with sensible defaults.
        /// Use `.with_<attribute>` methods to set specific values for tests.
        pub fn create_mock() -> Self {
            CombatData {
                units: HashMap::new(),
                unit_positions: HashMap::new(),
                current_turn: CombatTurn::Undefined,
            }
        }

        pub fn with_units(mut self, units: Vec<CombatUnit>) -> Self {
            for unit in units {
                self.unit_positions.insert(unit.position, unit.id);
                self.units.insert(unit.id, unit);
            }

            self
        }

        pub fn with_unit_turn(mut self, unit_id: UnitId) -> Self {
            self.current_turn = CombatTurn::start_unit_turn(&self.units[&unit_id]);
            self
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::combat_data::CombatData;
    use crate::combat_turn::{CombatTurn, UnitTurn};
    use crate::combat_unit::CombatUnit;
    use crate::unit_stats::UnitStats;

    #[test]
    fn test_start_unit_turn() {
        let mut combat_state = CombatData::create_mock().with_units(vec![
            CombatUnit::create_mock(1, 1).with_stats(UnitStats::create_mock().with_movement(5)),
            CombatUnit::create_mock(2, 1).with_stats(UnitStats::create_mock().with_movement(10)),
            CombatUnit::create_mock(3, 1).with_stats(UnitStats::create_mock().with_movement(15)),
        ]);

        combat_state.start_unit_turn(1);
        assert_eq!(
            CombatTurn::UnitTurn(UnitTurn {
                unit_id: 1,
                remaining_movement: 5,
                remaining_actions: 1,
            }),
            combat_state.current_turn
        );

        combat_state.start_unit_turn(2);
        assert_eq!(
            CombatTurn::UnitTurn(UnitTurn {
                unit_id: 2,
                remaining_movement: 10,
                remaining_actions: 1,
            }),
            combat_state.current_turn
        );

        combat_state.start_unit_turn(3);
        assert_eq!(
            CombatTurn::UnitTurn(UnitTurn {
                unit_id: 3,
                remaining_movement: 15,
                remaining_actions: 1,
            }),
            combat_state.current_turn
        );
    }
}
