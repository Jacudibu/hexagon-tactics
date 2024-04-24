use crate::map::highlighted_tiles;
use crate::map::highlighted_tiles::{RangeHighlightMarker, RangeHighlights};
use crate::map::map_gizmos::MapGizmosPlugin;
use crate::map::map_ui::MapUiPlugin;
use crate::map::spawning::MapSpawningPlugin;
use crate::map::tile_cursor::TileCursorPlugin;
use crate::map::update_tile::TileUpdaterPlugin;
use bevy::app::{App, Plugin};
use bevy::prelude::{
    resource_changed_or_removed, Component, Entity, IntoSystemConfigs, Reflect, Resource, States,
    Update,
};
use bevy::utils::HashMap;
use hexx::Hex;

pub struct GameMapPlugin;

impl Plugin for GameMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MapState>();
        app.add_plugins(TileCursorPlugin);
        app.add_plugins(MapGizmosPlugin);
        app.add_plugins(MapUiPlugin);
        app.add_plugins(MapSpawningPlugin);
        app.add_plugins(TileUpdaterPlugin);
        app.add_systems(
            Update,
            highlighted_tiles::on_highlight_change::<RangeHighlightMarker, RangeHighlights>
                .run_if(resource_changed_or_removed::<RangeHighlights>()),
        );
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Reflect)]
pub enum MapState {
    #[default]
    Unloaded,
    Loaded,
}

#[derive(Component, Debug)]
pub struct HexagonTileComponent {
    pub hex: Hex,
}

#[derive(Debug, Resource)]
pub struct MapTileEntities {
    pub parent: Entity,
    pub entities: HashMap<Hex, MapTileEntityBundle>,
}

#[derive(Debug)]
pub struct MapTileEntityBundle {
    pub parent: Entity,
    pub top: Entity,
    pub side: Option<Entity>,
    pub fluid: Option<Entity>,
}
