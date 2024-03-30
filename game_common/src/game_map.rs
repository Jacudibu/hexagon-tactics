use bevy::math::Vec2;
use bevy::prelude::{default, Resource};
use bevy::utils::hashbrown::HashMap;
use hexx::{Hex, HexLayout, HexOrientation};

#[derive(Debug, Resource)]
pub struct GameMap {
    pub layout: HexLayout,
    pub tiles: HashMap<Hex, TileData>,
}

pub const MIN_HEIGHT: u8 = 1;
pub const MAX_HEIGHT: u8 = 20;

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

        let layout = HexLayout {
            origin: Vec2::ZERO,
            hex_size: Vec2::splat(1.0),
            orientation: HexOrientation::Pointy,
            ..default()
        };

        GameMap { tiles, layout }
    }
}

#[derive(Debug)]
pub struct TileData {
    pub height: u8,
}
