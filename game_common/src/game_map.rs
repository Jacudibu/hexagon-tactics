use bevy::math::Vec2;
use bevy::prelude::Resource;
use bevy::utils::hashbrown::HashMap;
use hexx::{Hex, HexLayout, HexOrientation};
use std::fmt::{Display, Formatter};

#[derive(Debug, Resource)]
pub struct GameMap {
    pub tiles: HashMap<Hex, TileData>,
}

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
        for hex in hexx::shapes::hexagon(Hex::ORIGIN, radius) {
            tiles.insert(
                hex,
                TileData {
                    height: 1,
                    surface: TileSurface::Grass,
                },
            );
        }

        GameMap { tiles }
    }
}

#[derive(Debug)]
pub struct TileData {
    pub height: u8,
    pub surface: TileSurface,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub enum TileSurface {
    Grass,
    Stone,
    Sand,
    Earth,
    Water,
}

impl Display for TileSurface {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TileSurface::Grass => write!(f, "Grass"),
            TileSurface::Stone => write!(f, "Stone"),
            TileSurface::Sand => write!(f, "Sand"),
            TileSurface::Earth => write!(f, "Earth"),
            TileSurface::Water => write!(f, "Water"),
        }
    }
}
