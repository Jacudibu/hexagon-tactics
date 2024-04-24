mod highlights;
mod map_gizmos;
mod map_plugin;
mod map_ui;
mod spawning;
mod tile_cursor;
mod update_tile;

pub const METERS_PER_TILE_HEIGHT_UNIT: f32 = 0.5;
pub use {
    highlights::attack_highlights::AttackHighlights, highlights::range_highlights::RangeHighlights,
    map_plugin::GameMapPlugin, map_plugin::MapState, spawning::DespawnMapCommand,
    spawning::SpawnMapCommand, tile_cursor::MouseCursorOnTile, update_tile::TileChangeEvent,
};
