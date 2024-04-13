use crate::map::{DespawnMapCommand, MapState, SpawnMapCommand, TileChangeEvent, TileCursor};
use crate::map_editor::editor_ui::MapEditorUiPlugin;
use crate::{ApplicationState, MouseCursorOverUiState};
use bevy::app::{App, Plugin, Update};
use bevy::log::{error, warn};
use bevy::prelude::{
    in_state, Commands, EventWriter, IntoSystemConfigs, KeyCode, Local, MouseButton, OnEnter,
    OnExit, Query, Reflect, Res, ResMut, Resource,
};
use game_common::game_map::{Fluid, FluidKind, GameMap, TileData, TileSurface, MAX_HEIGHT};
use hexx::Hex;
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::axislike::DualAxis;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::prelude::InputKind;
use leafwing_input_manager::Actionlike;
use std::fmt::Formatter;
use std::ops::DerefMut;

pub struct MapEditorPlugin;

impl Plugin for MapEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<MapEditorAction>::default());
        app.add_plugins(MapEditorUiPlugin);
        app.init_resource::<ActionState<MapEditorAction>>();
        app.insert_resource(MapEditorAction::default_input_map());
        app.add_systems(OnEnter(ApplicationState::MapEditor), setup_map_editor);
        app.add_systems(OnExit(ApplicationState::MapEditor), exit_map_editor);
        app.add_systems(
            Update,
            (
                track_input,
                use_tool
                    .after(track_input)
                    .run_if(in_state(MouseCursorOverUiState::NotOverUI)),
            )
                .run_if(in_state(ApplicationState::MapEditor))
                .run_if(in_state(MapState::Loaded)),
        );
    }
}

fn setup_map_editor(mut commands: Commands, mut spawn_map_command: EventWriter<SpawnMapCommand>) {
    commands.insert_resource(MapEditorTool::default());
    let default_radius = 10;
    let map = GameMap::new(default_radius);
    commands.insert_resource(map);

    spawn_map_command.send(SpawnMapCommand {});
}

fn exit_map_editor(mut despawn_map_command: EventWriter<DespawnMapCommand>) {
    despawn_map_command.send(DespawnMapCommand {});
}

#[derive(Resource, Debug, Default)]
pub enum MapEditorTool {
    #[default]
    RaiseTiles,
    LowerTiles,
    PaintSurface(TileSurface),
    RaiseFluid(FluidKind),
    LowerFluid,
}

impl std::fmt::Display for MapEditorTool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MapEditorTool::RaiseTiles => write!(f, "Raise Tiles"),
            MapEditorTool::LowerTiles => write!(f, "Lower Tiles"),
            MapEditorTool::PaintSurface(surface) => write!(f, "Paint {}", surface),
            MapEditorTool::RaiseFluid(fluid) => write!(f, "Fill with {}", fluid),
            MapEditorTool::LowerFluid => write!(f, "Lower Fluid"),
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum MapEditorAction {
    UseTool,
    RaiseTiles,
    LowerTiles,
    PaintGrass,
    PaintStone,
    PaintSand,
    PaintEarth,
    RaiseWater,
    LowerFluid,
    MouseMotion,
}

impl std::fmt::Display for MapEditorAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MapEditorAction::UseTool => write!(f, "Use Tool"),
            MapEditorAction::RaiseTiles => write!(f, "Raise Tiles"),
            MapEditorAction::LowerTiles => write!(f, "Lower Tiles"),
            MapEditorAction::PaintGrass => write!(f, "Paint Grass"),
            MapEditorAction::PaintStone => write!(f, "Paint Stone"),
            MapEditorAction::PaintSand => write!(f, "Paint Sand"),
            MapEditorAction::PaintEarth => write!(f, "Paint Earth"),
            MapEditorAction::RaiseWater => write!(f, "Raise Water"),
            MapEditorAction::LowerFluid => write!(f, "Lower Water"),
            MapEditorAction::MouseMotion => {
                warn!(
                    "MapEditorAction::MouseMotion::Display was called. This should never happen?"
                );
                write!(f, "Mouse Motion")
            }
        }
    }
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
        input_map.insert(Self::RaiseWater, KeyCode::Digit5);
        input_map.insert(Self::LowerFluid, KeyCode::Digit6);
        input_map.insert(Self::MouseMotion, DualAxis::mouse_motion());

        input_map
    }
}

#[rustfmt::skip]
pub const ACTION_TO_TOOL: [(MapEditorAction, MapEditorTool); 8] = [
    (MapEditorAction::RaiseTiles, MapEditorTool::RaiseTiles),
    (MapEditorAction::LowerTiles, MapEditorTool::LowerTiles),
    (MapEditorAction::PaintGrass, MapEditorTool::PaintSurface(TileSurface::Grass)),
    (MapEditorAction::PaintStone, MapEditorTool::PaintSurface(TileSurface::Stone)),
    (MapEditorAction::PaintSand,  MapEditorTool::PaintSurface(TileSurface::Sand)),
    (MapEditorAction::PaintEarth, MapEditorTool::PaintSurface(TileSurface::Earth)),
    (MapEditorAction::RaiseWater, MapEditorTool::RaiseFluid(FluidKind::Water)),
    (MapEditorAction::LowerFluid, MapEditorTool::LowerFluid),
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

#[derive(Default)]
struct MultiselectData {
    total_mouse_delta: f32,
    previously_selected_tiles: Vec<Hex>,
}

impl MultiselectData {
    fn clear(&mut self) {
        self.total_mouse_delta = 0.0;
        self.previously_selected_tiles.clear();
    }
}

fn use_tool(
    map: ResMut<GameMap>,
    active_tool: Res<MapEditorTool>,
    current_selection: Query<&TileCursor>,
    input_state: Res<ActionState<MapEditorAction>>,
    mut multiselect_data: Local<MultiselectData>,
    mut tile_change_event: EventWriter<TileChangeEvent>,
) {
    if !input_state.pressed(&MapEditorAction::UseTool) {
        return;
    }

    if input_state.just_pressed(&MapEditorAction::UseTool) {
        multiselect_data.deref_mut().clear();
        create_tool_events_for_tile(
            map,
            &active_tool,
            current_selection,
            multiselect_data,
            &mut tile_change_event,
        );

        return;
    }

    if let Some(mouse_motion) = input_state.axis_pair(&MapEditorAction::MouseMotion) {
        multiselect_data.total_mouse_delta += mouse_motion.length();
        if multiselect_data.total_mouse_delta > 35.0 {
            create_tool_events_for_tile(
                map,
                &active_tool,
                current_selection,
                multiselect_data,
                &mut tile_change_event,
            );
        }
    }
}

fn create_tool_events_for_tile(
    mut map: ResMut<GameMap>,
    active_tool: &Res<MapEditorTool>,
    current_selection: Query<&TileCursor>,
    mut multiselect_data: Local<MultiselectData>,
    tile_change_event: &mut EventWriter<TileChangeEvent>,
) {
    tile_change_event.send_batch(current_selection.iter().filter_map(|x| {
        if multiselect_data.previously_selected_tiles.contains(&x.hex) {
            None
        } else {
            multiselect_data.previously_selected_tiles.push(x.hex);
            if let Some(tile) = map.tiles.get_mut(&x.hex) {
                if can_tool_be_used_on_tile(&active_tool, tile) {
                    let old_data = tile.clone();
                    use_tool_on_tile(&active_tool, tile);
                    Some(TileChangeEvent {
                        hex: x.hex,
                        old_data,
                    })
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
        MapEditorTool::RaiseFluid(_) => true, // TODO: Check tile.height + fluid.height against MAX_HEIGHT
        MapEditorTool::LowerFluid => tile.fluid.is_some(),
    }
}

fn use_tool_on_tile(tool: &MapEditorTool, tile: &mut TileData) {
    match tool {
        MapEditorTool::RaiseTiles => {
            tile.height += 1;
            if let Some(ref mut fluid) = tile.fluid {
                fluid.height -= 1.0;

                if fluid.height < 0.0 {
                    tile.fluid = None;
                }
            }
        }
        MapEditorTool::LowerTiles => {
            tile.height -= 1;
            if let Some(ref mut fluid) = tile.fluid {
                fluid.height += 1.0;
            }
        }
        MapEditorTool::PaintSurface(surface) => tile.surface = surface.clone(),
        MapEditorTool::RaiseFluid(kind) => {
            if let Some(ref mut fluid) = tile.fluid {
                if &fluid.kind == kind {
                    fluid.height += 1.0;
                } else {
                    fluid.kind = kind.clone();
                }
            } else {
                tile.fluid = Some(Fluid {
                    kind: kind.clone(),
                    height: 0.75,
                })
            }
        }
        MapEditorTool::LowerFluid => {
            if let Some(ref mut fluid) = tile.fluid {
                fluid.height -= 1.0;
                if fluid.height <= 0.0 {
                    tile.fluid = None;
                }
            }
        }
    }
}
