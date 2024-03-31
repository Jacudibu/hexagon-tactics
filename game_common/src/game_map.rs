use bevy::math::Vec2;
use bevy::prelude::Resource;
use bevy::utils::hashbrown::HashMap;
use hexx::{Hex, HexLayout, HexOrientation};

#[derive(Debug, Resource)]
pub struct GameMap {
    pub tiles: HashMap<Hex, TileData>,
}

pub const MIN_HEIGHT: u8 = 1;
pub const MAX_HEIGHT: u8 = 20;
pub const HEX_LAYOUT: HexLayout = HexLayout {
    hex_size: Vec2::splat(1.0),
    origin: Vec2::ZERO,
    orientation: HexOrientation::Pointy,
    invert_x: false,
    invert_y: false,
};

impl GameMap {
    pub fn new(radius: u32) -> Self {
        let mut tiles = HashMap::new();
        let mut height = MIN_HEIGHT;
        for hex in hexx::shapes::hexagon(Hex::ORIGIN, radius) {
            tiles.insert(hex, TileData { height });
            if height == MAX_HEIGHT {
                height = MIN_HEIGHT;
            } else {
                height += 1;
            }
        }

        GameMap { tiles }
    }
}

#[derive(Debug)]
pub struct TileData {
    pub height: u8,
}
