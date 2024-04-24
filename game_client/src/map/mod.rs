mod highlighted_tiles;
mod map_gizmos;
mod map_plugin;
mod map_ui;
mod spawning;
mod tile_cursor;
mod update_tile;

pub const METERS_PER_TILE_HEIGHT_UNIT: f32 = 0.5;
pub use {
    highlighted_tiles::RangeHighlights, map_plugin::GameMapPlugin, map_plugin::MapState,
    spawning::DespawnMapCommand, spawning::SpawnMapCommand, tile_cursor::MouseCursorOnTile,
    tile_cursor::TileCursor, update_tile::TileChangeEvent,
};
