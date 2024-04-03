use std::fmt::Formatter;
use std::ops::DerefMut;

use bevy::app::App;
use bevy::prelude::*;
use hexx::Hex;
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::prelude::InputKind;
use leafwing_input_manager::Actionlike;

use game_common::game_map::{GameMap, TileData, TileSurface, MAX_HEIGHT};

use crate::game_map::editor::editor_ui::EditorUiPlugin;
use crate::game_map::tile_cursor::TileCursor;
use crate::game_map::{HexagonMaterials, HexagonMeshes, MapTileEntities};

mod editor_ui;

pub struct MapEditorPlugin;
impl Plugin for MapEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<MapEditorAction>::default());
        app.add_plugins(EditorUiPlugin);
        app.init_resource::<ActionState<MapEditorAction>>();
        app.insert_resource(MapEditorAction::default_input_map());
        app.insert_resource(MapEditorTool::default());
        app.add_systems(
            Update,
            (
                track_input,
                use_tool.after(track_input),
                update_tile_entity.after(use_tool),
            ),
        );
        app.add_event::<TileChangeEvent>();
    }
}

#[derive(Resource, Debug, Default)]
enum MapEditorTool {
    #[default]
    RaiseTiles,
    LowerTiles,
    PaintSurface(TileSurface),
}

impl std::fmt::Display for MapEditorTool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MapEditorTool::RaiseTiles => write!(f, "Raise Tiles"),
            MapEditorTool::LowerTiles => write!(f, "Lower Tiles"),
            MapEditorTool::PaintSurface(surface) => write!(f, "Paint {}", surface),
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum MapEditorAction {
    UseTool,
    RaiseTiles,
    LowerTiles,
    PaintGrass,
    PaintStone,
    PaintSand,
    PaintEarth,
    PaintWater,
}

impl MapEditorAction {
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();
        input_map.insert(Self::UseTool, InputKind::Mouse(MouseButton::Left));
        input_map.insert(Self::RaiseTiles, KeyCode::KeyR);
        input_map.insert(Self::LowerTiles, KeyCode::KeyF);
        input_map.insert(Self::PaintGrass, KeyCode::Digit1);
        input_map.insert(Self::PaintStone, KeyCode::Digit2);
        input_map.insert(Self::PaintSand, KeyCode::Digit3);
        input_map.insert(Self::PaintEarth, KeyCode::Digit4);
        input_map.insert(Self::PaintWater, KeyCode::Digit5);

        input_map
    }
}

#[rustfmt::skip]
const ACTION_TO_TOOL: [(MapEditorAction, MapEditorTool); 7] = [
    (MapEditorAction::RaiseTiles, MapEditorTool::RaiseTiles),
    (MapEditorAction::LowerTiles, MapEditorTool::LowerTiles),
    (MapEditorAction::PaintGrass, MapEditorTool::PaintSurface(TileSurface::Grass)),
    (MapEditorAction::PaintStone, MapEditorTool::PaintSurface(TileSurface::Stone)),
    (MapEditorAction::PaintSand,  MapEditorTool::PaintSurface(TileSurface::Sand)),
    (MapEditorAction::PaintEarth, MapEditorTool::PaintSurface(TileSurface::Earth)),
    (MapEditorAction::PaintWater, MapEditorTool::PaintSurface(TileSurface::Water)),
];

fn track_input(
    input_state: Res<ActionState<MapEditorAction>>,
    mut current_tool: ResMut<MapEditorTool>,
) {
    for (action, tool) in ACTION_TO_TOOL {
        if input_state.just_pressed(&action) {
            *current_tool = tool;
            break;
        }
    }
}

fn use_tool(
    mut map: ResMut<GameMap>,
    active_tool: Res<MapEditorTool>,
    current_selection: Query<&TileCursor>,
    input_state: Res<ActionState<MapEditorAction>>,
    mut previously_interacted_tiles: Local<Vec<Hex>>,
    mut tile_change_event: EventWriter<TileChangeEvent>,
) {
    if !input_state.pressed(&MapEditorAction::UseTool) {
        return;
    }

    if input_state.just_pressed(&MapEditorAction::UseTool) {
        previously_interacted_tiles.deref_mut().clear();
    }

    tile_change_event.send_batch(current_selection.iter().filter_map(|x| {
        if previously_interacted_tiles.contains(&x.hex) {
            None
        } else {
            previously_interacted_tiles.push(x.hex);
            if let Some(tile) = map.tiles.get_mut(&x.hex) {
                if can_tool_be_used_on_tile(&active_tool, tile) {
                    use_tool_on_tile(&active_tool, tile);
                    Some(TileChangeEvent { hex: x.hex })
                } else {
                    None
                }
            } else {
                error!("Was unable to find hex tile_data at {:?} in map!", x);
                None
            }
        }
    }));
}

#[must_use]
fn can_tool_be_used_on_tile(tool: &MapEditorTool, tile: &TileData) -> bool {
    match tool {
        MapEditorTool::RaiseTiles => tile.height < MAX_HEIGHT,
        MapEditorTool::LowerTiles => tile.height > 0,
        MapEditorTool::PaintSurface(_) => true,
    }
}

fn use_tool_on_tile(tool: &MapEditorTool, mut tile: &mut TileData) {
    match tool {
        MapEditorTool::RaiseTiles => tile.height += 1,
        MapEditorTool::LowerTiles => tile.height -= 1,
        MapEditorTool::PaintSurface(surface) => tile.surface = surface.clone(),
    }
}

fn update_tile_entity(
    mut commands: Commands,
    map: Res<GameMap>,
    mut tile_change_event: EventReader<TileChangeEvent>,
    meshes: Res<HexagonMeshes>,
    materials: Res<HexagonMaterials>,
    tile_entities: Res<MapTileEntities>,
) {
    for event in tile_change_event.read() {
        if let Some(tile) = map.tiles.get(&event.hex) {
            if let Some(entity) = tile_entities.entities.get(&event.hex) {
                let mut commands = commands.entity(*entity);
                if let Some(mesh) = meshes.columns.get(&tile.height) {
                    commands.insert(mesh.clone());
                    // FIXME: Temporary fix for https://github.com/bevyengine/bevy/issues/4294 and/or https://github.com/aevyrie/bevy_mod_raycast/issues/42
                    commands.remove::<bevy::render::primitives::Aabb>();
                } else {
                    error!("Was unable to find hex mesh for height {}!", tile.height);
                }

                if tile.height == 0 {
                    commands.insert(materials.invisible.clone());
                } else {
                    commands.insert(materials.surface_material(&tile.surface));
                }
            } else {
                error!("Was unable to find hex entity at {:?} in map!", event.hex);
            }
        } else {
            error!(
                "Was unable to find hex tile_data at {:?} in map!",
                event.hex
            );
        }
    }
}

#[derive(Event)]
pub struct TileChangeEvent {
    pub hex: Hex,
}
