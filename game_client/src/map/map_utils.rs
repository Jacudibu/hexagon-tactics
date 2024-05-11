use crate::map::METERS_PER_TILE_HEIGHT_UNIT;
use bevy::log::error;
use bevy::math::Vec3;
use game_common::game_map::{GameMap, HEX_LAYOUT};
use hexx::Hex;

pub fn unit_position_on_hexagon(hex: Hex, map: &GameMap) -> Vec3 {
    let height = match map.tiles.get(&hex) {
        None => {
            error!(
                "Was unable to find tile for hex when solving unit position: {:?}",
                hex
            );
            0.0
        }
        Some(tile_data) => tile_data.height as f32 * METERS_PER_TILE_HEIGHT_UNIT,
    };

    let hex_pos = HEX_LAYOUT.hex_to_world_pos(hex);

    Vec3::new(hex_pos.x, height, hex_pos.y)
}
