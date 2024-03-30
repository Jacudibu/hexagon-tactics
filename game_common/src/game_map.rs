use bevy::prelude::Resource;
use bevy::utils::hashbrown::HashMap;
use hexx::Hex;

#[derive(Debug, Resource)]
pub struct GameMap {
    pub tiles: HashMap<Hex, TileData>,
}

impl GameMap {
    pub fn new(radius: u32) -> Self {
        let mut tiles = HashMap::new();
        for hex in hexx::shapes::hexagon(Hex::ORIGIN, radius) {
            tiles.insert(hex, TileData { height: 0 });
        }

        GameMap { tiles }
    }
}

#[derive(Debug)]
pub struct TileData {
    pub height: u8,
}
