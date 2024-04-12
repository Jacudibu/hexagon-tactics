use crate::game_map::GameMap;
use crate::units::{Unit, UnitId};
use bevy::prelude::{error, Resource};
use bevy::utils::HashMap;
use hexx::Hex;

#[derive(Resource, PartialEq, Debug)]
pub struct CombatData {
    pub units: HashMap<UnitId, Unit>,
    pub unit_positions: HashMap<Hex, UnitId>,
    pub turn_order: HashMap<u32, UnitId>,
    pub units_that_can_still_be_placed: Vec<UnitId>,
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
            if fluid.height > 0.5 {
                return false;
            }
        }

        true
    }
}

#[derive(Debug)]
pub struct ValidationError {
    message: String,
}

impl ValidationError {
    pub fn new(message: &str) -> Self {
        ValidationError {
            message: message.into(),
        }
    }
}
