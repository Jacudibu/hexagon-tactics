use crate::game_map::GameMap;
use crate::unit::{Unit, UnitId};
use crate::validation_error::ValidationError;
use bevy::prelude::{error, Resource};
use bevy::utils::HashMap;
use hexx::Hex;

/// Combat Data shared between client and server. Things in here should always be kept in sync.
#[derive(Resource, PartialEq, Debug)]
pub struct CombatData {
    pub units: HashMap<UnitId, Unit>,
    pub unit_positions: HashMap<Hex, UnitId>,
    pub turn_order: HashMap<u32, UnitId>, // TODO: Turn this into a Vec<(Speed/u32, UnitId)>, maybe calculate & store locally
    pub unit_storage: Vec<Unit>,
    pub current_unit_turn: Option<UnitId>, // TODO: That's just the first inside turn_order, and if the server tells us to start the turn for a different unit, something is terribly wrong.
}

impl CombatData {
    pub fn validate_path(
        &self,
        map: &GameMap,
        start: &Hex,
        path: Vec<&Hex>,
        unit: &Unit,
    ) -> Result<(), ValidationError> {
        if (unit.turn_resources.remaining_movement as usize) < path.len() {
            return Err(ValidationError::new("Not enough movement."));
        }

        let mut last_height = match map.tiles.get(start) {
            None => {
                return Err(ValidationError::new("Starting Tile doesn't exist?"));
            }
            Some(tile_data) => tile_data.height,
        };
        for hex in path {
            match map.tiles.get(hex) {
                None => {
                    return Err(ValidationError::new("Tile doesn't exist?"));
                }
                Some(tile_data) => {
                    if tile_data.height == 0 {
                        return Err(ValidationError::new("Tile isn't walkable!"));
                    }

                    if let Some(fluid) = &tile_data.fluid {
                        if fluid.height > 1.0 {
                            return Err(ValidationError::new("Tile fluid is too deep!"));
                        }
                    }

                    let difference = if tile_data.height > last_height {
                        tile_data.height - last_height
                    } else {
                        last_height - tile_data.height
                    };

                    if difference > unit.stats_after_buffs.jump {
                        return Err(ValidationError::new("Unit can't jump high enough!"));
                    }

                    last_height = tile_data.height;
                }
            };
        }

        Ok(())
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
            }
        }

        pub fn with_units(mut self, units: Vec<Unit>) -> Self {
            for unit in units {
                self.unit_positions.insert(unit.position.unwrap(), unit.id);
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
