use crate::game_map::GameMap;
use crate::turn_resources::TurnResources;
use crate::unit::{Unit, UnitId};
use bevy::prelude::{error, Resource};
use bevy::utils::HashMap;
use hexx::Hex;

/// Combat Data shared between client and server. Things in here should always be kept in sync.
#[derive(Resource, Debug)]
pub struct CombatData {
    pub units: HashMap<UnitId, Unit>,
    pub unit_positions: HashMap<Hex, UnitId>,
    pub unit_storage: Vec<Unit>,
    pub current_unit_turn: Option<UnitId>,
    pub turn_resources: TurnResources,
}

impl CombatData {
    pub fn start_unit_turn(&mut self, unit_id: UnitId) {
        self.current_unit_turn = Some(unit_id);

        let unit = &self.units[&unit_id];
        self.turn_resources.remaining_movement = unit.stats_after_buffs.movement;
    }

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

    pub fn get_next_unit(&mut self) -> UnitId {
        let mut ready_units = Vec::new();
        let mut highest_counter = Self::COUNTER_NEEDED_FOR_TURN;
        for x in self.units.values() {
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
    fn from(unit: &Unit) -> TurnOrderElement {
        TurnOrderElement {
            tiebreaker: unit.turn_tiebreaker,
            unit_id: unit.id,
        }
    }
}

#[cfg(feature = "test_helpers")]
pub mod test_helpers {
    use crate::combat_data::CombatData;
    use crate::turn_resources::TurnResources;
    use crate::unit::{Unit, UnitId};
    use bevy::utils::HashMap;

    impl CombatData {
        /// Create mock CombatData with sensible defaults.
        /// Use `.with_<attribute>` methods to set specific values for tests.
        pub fn create_mock() -> Self {
            CombatData {
                units: HashMap::new(),
                unit_positions: HashMap::new(),
                unit_storage: Vec::new(),
                current_unit_turn: None,
                turn_resources: TurnResources {
                    remaining_movement: 0,
                },
            }
        }

        pub fn with_units(mut self, units: Vec<Unit>) -> Self {
            for unit in units {
                self.unit_positions.insert(unit.position, unit.id);
                self.units.insert(unit.id, unit);
            }

            self
        }

        pub fn with_unit_turn(mut self, unit_id: UnitId) -> Self {
            self.current_unit_turn = Some(unit_id);
            self
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::combat_data::CombatData;
    use crate::unit::Unit;
    use crate::unit_stats::UnitStats;

    #[test]
    fn test_start_unit_turn() {
        let mut combat_state = CombatData::create_mock().with_units(vec![
            Unit::create_mock(1, 1).with_stats(UnitStats::create_mock().with_movement(5)),
            Unit::create_mock(2, 1).with_stats(UnitStats::create_mock().with_movement(10)),
            Unit::create_mock(3, 1).with_stats(UnitStats::create_mock().with_movement(15)),
        ]);

        combat_state.start_unit_turn(1);
        assert_eq!(Some(1), combat_state.current_unit_turn);
        assert_eq!(5, combat_state.turn_resources.remaining_movement);

        combat_state.start_unit_turn(2);
        assert_eq!(Some(2), combat_state.current_unit_turn);
        assert_eq!(10, combat_state.turn_resources.remaining_movement);

        combat_state.start_unit_turn(3);
        assert_eq!(Some(3), combat_state.current_unit_turn);
        assert_eq!(15, combat_state.turn_resources.remaining_movement);
    }
}
