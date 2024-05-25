use std::fmt::Formatter;
use std::ops::DerefMut;

use bevy::log::error;
use bevy::prelude::{EventWriter, Local, Res, ResMut, Resource};
use leafwing_input_manager::action_state::ActionState;

use game_common::game_map::{Fluid, FluidKind, GameMap, TileData, TileSurface, MAX_HEIGHT};

use crate::map::{CursorOnTile, TileChangeEvent};
use crate::map_editor::map_editor_action::MapEditorAction;
use crate::map_editor::multiselect_data::MultiselectData;

#[derive(Resource, Debug, Default)]
pub enum MapEditorTool {
    #[default]
    RaiseTiles,
    LowerTiles,
    PaintSurface(TileSurface),
    RaiseFluid(FluidKind),
    LowerFluid,
    MarkSpawnTile(u8),
    RemoveSpawnTile,
}

impl std::fmt::Display for MapEditorTool {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            MapEditorTool::RaiseTiles => write!(f, "Raise Tiles"),
            MapEditorTool::LowerTiles => write!(f, "Lower Tiles"),
            MapEditorTool::PaintSurface(surface) => write!(f, "Paint {}", surface),
            MapEditorTool::RaiseFluid(fluid) => write!(f, "Fill with {}", fluid),
            MapEditorTool::LowerFluid => write!(f, "Lower Fluid"),
            MapEditorTool::MarkSpawnTile(team) => write!(f, "Mark Spawn Tiles for Team {team}"),
            MapEditorTool::RemoveSpawnTile => write!(f, "Remove Spawn Tiles"),
        }
    }
}

pub fn use_tool(
    map: ResMut<GameMap>,
    active_tool: Res<MapEditorTool>,
    cursor: Option<Res<CursorOnTile>>,
    input_state: Res<ActionState<MapEditorAction>>,
    mut multiselect_data: Local<MultiselectData>,
    mut tile_change_event: EventWriter<TileChangeEvent>,
) {
    let Some(cursor) = cursor else {
        return;
    };

    if !input_state.pressed(&MapEditorAction::UseTool) {
        return;
    }

    if input_state.just_pressed(&MapEditorAction::UseTool) {
        multiselect_data.deref_mut().clear();
        create_tool_events_for_tile(
            map,
            &active_tool,
            &cursor,
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
                &cursor,
                multiselect_data,
                &mut tile_change_event,
            );
        }
    }
}

fn create_tool_events_for_tile(
    mut map: ResMut<GameMap>,
    active_tool: &Res<MapEditorTool>,
    cursor: &CursorOnTile,
    mut multiselect_data: Local<MultiselectData>,
    tile_change_event: &mut EventWriter<TileChangeEvent>,
) {
    let cursor = std::iter::once(cursor);
    tile_change_event.send_batch(cursor.filter_map(|cursor| {
        if multiselect_data
            .previously_selected_tiles
            .contains(&cursor.hex)
        {
            None
        } else {
            multiselect_data.previously_selected_tiles.push(cursor.hex);
            if let Some(tile) = map.tiles.get_mut(&cursor.hex) {
                if can_tool_be_used_on_tile(&active_tool, tile) {
                    let old_data = tile.clone();
                    use_tool_on_tile(&active_tool, tile);
                    Some(TileChangeEvent {
                        hex: cursor.hex,
                        old_data,
                    })
                } else {
                    None
                }
            } else {
                error!("Was unable to find hex tile_data at {:?} in map!", cursor);
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
        MapEditorTool::MarkSpawnTile(_) => tile.height > 0,
        MapEditorTool::RemoveSpawnTile => true,
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
        MapEditorTool::MarkSpawnTile(team) => tile.spawn_zone = Some(team.clone()),
        MapEditorTool::RemoveSpawnTile => tile.spawn_zone = None,
    }
}
