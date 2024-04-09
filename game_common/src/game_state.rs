use crate::game_map::GameMap;
use crate::units::{Unit, UnitId};
use bevy::prelude::Resource;
use bevy::utils::HashMap;
use hexx::Hex;

#[derive(Resource, PartialEq, Debug)]
pub struct GameState {
    pub map: GameMap,
    pub units: HashMap<UnitId, Unit>,
    pub unit_positions: HashMap<Hex, UnitId>,
    pub turn_order: HashMap<u32, UnitId>,
    pub units_that_can_still_be_placed: Vec<Unit>,
}

impl GameState {
    pub fn validate_path(
        &self,
        start: &Hex,
        path: Vec<&Hex>,
        unit: &Unit,
    ) -> Result<(), ValidationError> {
        if (unit.turn_resources.remaining_movement as usize) < path.len() {
            return Err(ValidationError::new("Not enough movement."));
        }

        let mut last_height = match self.map.tiles.get(start) {
            None => {
                return Err(ValidationError::new("Starting Tile doesn't exist?"));
            }
            Some(tile_data) => tile_data.height,
        };
        for hex in path {
            match self.map.tiles.get(hex) {
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
