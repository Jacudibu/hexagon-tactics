use crate::game_map::tile_cursor::TileCursor;
use crate::game_map::{HexagonMeshes, MapTileEntities};
use crate::networking::ServerConnection;
use bevy::app::App;
use bevy::prelude::*;
use game_common::game_map::{GameMap, MAX_HEIGHT};

pub struct MapEditorPlugin;
impl Plugin for MapEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, use_tool);
    }
}

fn use_tool(
    mut commands: Commands,
    mut map: ResMut<GameMap>,
    tile_entities: Res<MapTileEntities>,
    meshes: Res<HexagonMeshes>,
    current_selection: Query<&TileCursor>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    // TODO: Use input manager
    if !keyboard_input.just_pressed(KeyCode::KeyX) {
        return;
    }

    for x in current_selection.iter() {
        if let Some(tile) = map.tiles.get_mut(&x.hex) {
            if tile.height < MAX_HEIGHT {
                tile.height += 1;
            }

            if let Some(entity) = tile_entities.entities.get(&x.hex) {
                if let Some(mesh) = meshes.columns.get(&tile.height) {
                    commands.entity(*entity).insert(mesh.clone());
                } else {
                    error!("Was unable to find hex mesh for height {}!", tile.height);
                }
            } else {
                error!("Was unable to find hex entity at {:?} in map!", x);
            }
        } else {
            error!("Was unable to find hex tile_data at {:?} in map!", x);
        }
    }
}
