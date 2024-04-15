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
    pub turn_order: HashMap<u32, UnitId>, // TODO: Turn this into a Vec<(Speed/u32, UnitId)>, maybe calculate & store locally
    pub unit_storage: Vec<Unit>,
    pub current_unit_turn: Option<UnitId>, // TODO: That's just the first inside turn_order, and if the server tells us to start the turn for a different unit, something is terribly wrong.
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
                turn_order: HashMap::new(),
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
                self.turn_order.insert(0, unit.id);
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
