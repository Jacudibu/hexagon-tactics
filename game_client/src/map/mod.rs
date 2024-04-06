use bevy::app::{App, Plugin};

use bevy::prelude::*;
use bevy::utils::HashMap;

use hexx::Hex;

pub use {
    spawning::DespawnMapCommand, spawning::SpawnMapCommand, tile_cursor::TileCursor,
    update_tile::TileChangeEvent,
};

use crate::load::HexagonMeshes;
use crate::map::map_gizmos::MapGizmosPlugin;
use crate::map::map_ui::MapUiPlugin;
use crate::map::spawning::MapSpawningPlugin;
use crate::map::tile_cursor::TileCursorPlugin;
use crate::map::update_tile::TileUpdaterPlugin;

mod map_gizmos;
mod map_ui;
mod spawning;
mod tile_cursor;
mod update_tile;

pub const METERS_PER_TILE_HEIGHT_UNIT: f32 = 0.5;

pub struct GameMapPlugin;
impl Plugin for GameMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MapState>();
        app.add_plugins(TileCursorPlugin);
        app.add_plugins(MapGizmosPlugin);
        app.add_plugins(MapUiPlugin);
        app.add_plugins(MapSpawningPlugin);
        app.add_plugins(TileUpdaterPlugin);
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Reflect)]
pub enum MapState {
    #[default]
    Unloaded,
    Loaded,
}

#[derive(Component, Debug)]
pub(in crate::map) struct HexagonTileComponent {
    pub hex: Hex,
}

#[derive(Debug, Resource)]
pub(in crate::map) struct MapTileEntities {
    pub parent: Entity,
    pub entities: HashMap<Hex, MapTileEntityBundle>,
}

#[derive(Debug)]
pub(in crate::map) struct MapTileEntityBundle {
    pub parent: Entity,
    pub top: Entity,
    pub side: Entity,
    pub fluid: Option<Entity>,
}
