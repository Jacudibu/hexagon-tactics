use crate::game_map::tile_cursor::TileCursor;
use crate::game_map::{HexagonMeshes, MapTileEntities};
use crate::networking::ServerConnection;
use bevy::app::App;
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use futures::future::err;
use game_common::game_map::{GameMap, TileData, MAX_HEIGHT};
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::Actionlike;

pub struct MapEditorPlugin;
impl Plugin for MapEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<MapEditorAction>::default());
        app.init_resource::<ActionState<MapEditorAction>>();
        app.insert_resource(MapEditorAction::default_input_map());
        app.insert_resource(MapEditorTool::default());
        app.add_systems(Update, (track_input, use_tool.after(track_input)));
    }
}

#[derive(Resource, Debug, Default)]
enum MapEditorTool {
    #[default]
    RaiseTiles,
    LowerTiles,
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum MapEditorAction {
    UseTool,
    RaiseTiles,
    LowerTiles,
}

impl MapEditorAction {
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        input_map.insert(Self::UseTool, KeyCode::KeyX);
        input_map.insert(Self::RaiseTiles, KeyCode::KeyR);
        input_map.insert(Self::LowerTiles, KeyCode::KeyF);

        input_map
    }
}

fn track_input(
    input_state: Res<ActionState<MapEditorAction>>,
    mut current_tool: ResMut<MapEditorTool>,
) {
    if input_state.just_pressed(&MapEditorAction::RaiseTiles) {
        *current_tool = MapEditorTool::RaiseTiles;
    } else if input_state.just_pressed(&MapEditorAction::LowerTiles) {
        *current_tool = MapEditorTool::LowerTiles;
    }

    // TODO: Decide if we want to introduce an event or just track that press in use_tool
}

fn use_tool(
    mut commands: Commands,
    mut map: ResMut<GameMap>,
    active_tool: Res<MapEditorTool>,
    tile_entities: Res<MapTileEntities>,
    meshes: Res<HexagonMeshes>,
    current_selection: Query<&TileCursor>,
    input_state: Res<ActionState<MapEditorAction>>,
) {
    if !input_state.just_pressed(&MapEditorAction::UseTool) {
        return;
    }

    error!("a");
    for x in current_selection.iter() {
        if let Some(tile) = map.tiles.get_mut(&x.hex) {
            if !can_tool_be_used_on_tile(&active_tool, tile) {
                continue;
            }
            error!("b");

            use_tool_on_tile(&active_tool, tile);

            if let Some(entity) = tile_entities.entities.get(&x.hex) {
                update_tile_entity(&mut commands, tile, entity, &meshes);
            } else {
                error!("Was unable to find hex entity at {:?} in map!", x);
            }
        } else {
            error!("Was unable to find hex tile_data at {:?} in map!", x);
        }
    }
}

#[must_use]
fn can_tool_be_used_on_tile(tool: &MapEditorTool, tile: &TileData) -> bool {
    match tool {
        MapEditorTool::RaiseTiles => tile.height < MAX_HEIGHT,
        MapEditorTool::LowerTiles => tile.height > 0,
    }
}

fn use_tool_on_tile(tool: &MapEditorTool, mut tile: &mut TileData) {
    match tool {
        MapEditorTool::RaiseTiles => tile.height += 1,
        MapEditorTool::LowerTiles => tile.height -= 1,
    }
}

fn update_tile_entity(
    mut commands: &mut Commands,
    tile: &TileData,
    entity: &Entity,
    meshes: &HexagonMeshes,
) {
    if let Some(mesh) = meshes.columns.get(&tile.height) {
        commands.entity(*entity).insert(mesh.clone());
    } else {
        error!("Was unable to find hex mesh for height {}!", tile.height);
    }
}
