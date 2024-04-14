use crate::game_map::fluid::Fluid;
use crate::game_map::tile_surface::TileSurface;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct TileData {
    pub height: u8,
    pub surface: TileSurface,
    pub fluid: Option<Fluid>,
}
