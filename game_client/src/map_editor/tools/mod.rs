use bevy::app::App;
use bevy::prelude::Plugin;

mod spawn_marker;

pub mod events {
    pub use spawn_marker::{AddSpawnMarkerEvent, RemoveSpawnMarkerEvent};

    use crate::map_editor::tools::spawn_marker;
}

pub struct MapEditorToolsPlugin;
impl Plugin for MapEditorToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(spawn_marker::SpawnMarkerToolPlugin);
    }
}
