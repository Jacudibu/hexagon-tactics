mod field_of_movement_with_edge_detection;
mod fluid;
mod fluid_kind;
mod game_map;
mod tile_data;
mod tile_surface;
mod versioned_map_data;

use bevy::math::Vec2;
use hexx::{HexLayout, HexOrientation};

pub use {
    fluid::Fluid, fluid_kind::FluidKind, game_map::GameMap, tile_data::TileData,
    tile_surface::TileSurface,
};

pub const MAX_HEIGHT: u8 = 20;

pub const HEX_LAYOUT: HexLayout = HexLayout {
    hex_size: Vec2::splat(1.0),
    origin: Vec2::ZERO,
    orientation: HexOrientation::Pointy,
    invert_x: false,
    invert_y: false,
};
