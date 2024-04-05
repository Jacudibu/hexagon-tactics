mod versioned_map_data;

use bevy::math::Vec2;
use bevy::prelude::Resource;
use bevy::utils::hashbrown::HashMap;
use hexx::{Hex, HexLayout, HexOrientation};
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use versioned_map_data::VersionedMapData;

#[derive(Debug, Resource, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct GameMap {
    pub radius: u32,
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

        GameMap { radius, tiles }
    }

    pub fn write_to_disk(&self, path: &str) {
        // This clone is highly suboptimal.
        let _ = VersionedMapData::V1(self.clone()).write_to_disk(path);
    }

    pub fn load_from_file(path: &str) -> Result<Self, ()> {
        VersionedMapData::load_from_file(path)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Eq, PartialEq)]
pub struct TileData {
    pub height: u8,
    pub surface: TileSurface,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn saving_and_loading() {
        let mut map = GameMap::new(3);
        let hex_with_different_height = Hex::new(-1, 0);
        let hex_with_different_surface = Hex::new(1, 0);

        map.tiles
            .get_mut(&hex_with_different_height)
            .unwrap()
            .height = 4;
        map.tiles
            .get_mut(&hex_with_different_surface)
            .unwrap()
            .surface = TileSurface::Stone;

        let dir = TempDir::new().unwrap();
        let bla = dir.path().join("saving_and_loading");
        let path = bla.to_str().unwrap();

        // Round-trip: Load -> Save -> Load
        map.write_to_disk(path);
        let loaded_map = GameMap::load_from_file(path).unwrap();
        assert_eq!(loaded_map, map);

        // Round-trip: Save -> Load -> Save
        let bla = dir.path().join("saving_and_loading");
        let path2 = bla.to_str().unwrap();
        loaded_map.write_to_disk(path);

        assert_eq!(
            fs::read_to_string(path2).unwrap(),
            fs::read_to_string(path).unwrap()
        );
    }
}