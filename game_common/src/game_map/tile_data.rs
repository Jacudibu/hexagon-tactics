use crate::game_map::fluid::Fluid;
use crate::game_map::tile_surface::TileSurface;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TileData {
    pub height: u8,
    pub surface: TileSurface,
    pub fluid: Option<Fluid>,
    pub spawn_zone: Option<u8>,
}

impl TileData {
    pub fn can_unit_be_placed_here(&self, team: &u8) -> bool {
        let Some(spawn_team) = &self.spawn_zone else {
            return false;
        };

        if spawn_team != team {
            return false;
        }

        if let Some(fluid) = &self.fluid {
            if fluid.height > 1.0 {
                return false;
            }
        }

        true
    }
}
