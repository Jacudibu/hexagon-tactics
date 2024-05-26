use std::fmt::Formatter;
use std::ops::DerefMut;

use bevy::log::error;
use bevy::prelude::{Commands, Event, EventReader, EventWriter, Local, Res, ResMut, Resource};
use hexx::Hex;
use leafwing_input_manager::action_state::ActionState;

use game_common::game_map::{Fluid, FluidKind, GameMap, TileData, TileSurface, MAX_HEIGHT};

use crate::map::{CursorOnTile, RangeHighlights, TileChangeEvent};
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
    mut add_spawn_marker_event: EventWriter<AddSpawnMarkerEvent>,
    mut remove_spawn_marker_event: EventWriter<RemoveSpawnMarkerEvent>,
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
            &mut add_spawn_marker_event,
            &mut remove_spawn_marker_event,
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
                &mut add_spawn_marker_event,
                &mut remove_spawn_marker_event,
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
    add_spawn_marker_event: &mut EventWriter<AddSpawnMarkerEvent>,
    remove_spawn_marker_event: &mut EventWriter<RemoveSpawnMarkerEvent>,
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
                    use_tool_on_tile(
                        &active_tool,
                        &cursor.hex,
                        tile,
                        add_spawn_marker_event,
                        remove_spawn_marker_event,
                    );
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
        MapEditorTool::MarkSpawnTile(team) => {
            tile.height > 0 && !tile.spawn_zone.is_some_and(|x| &x == team)
        }
        MapEditorTool::RemoveSpawnTile => tile.spawn_zone.is_some(),
    }
}

fn use_tool_on_tile(
    tool: &MapEditorTool,
    hex: &Hex,
    tile: &mut TileData,
    add_spawn_marker_event: &mut EventWriter<AddSpawnMarkerEvent>,
    remove_spawn_marker_event: &mut EventWriter<RemoveSpawnMarkerEvent>,
) {
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
        MapEditorTool::MarkSpawnTile(team) => {
            tile.spawn_zone = Some(team.clone());
            add_spawn_marker_event.send(AddSpawnMarkerEvent {
                tile: hex.clone(),
                team: team.clone(),
            });
        }
        MapEditorTool::RemoveSpawnTile => {
            tile.spawn_zone = None;
            remove_spawn_marker_event.send(RemoveSpawnMarkerEvent { tile: hex.clone() });
        }
    }
}

pub fn on_tool_change(
    commands: Commands,
    map: Res<GameMap>,
    tool: Option<Res<MapEditorTool>>,
    highlights: Option<Res<RangeHighlights>>,
) {
    match tool {
        None => {
            remove_all_spawn_spot_markers(commands);
        }
        Some(tool) => match tool.into_inner() {
            MapEditorTool::MarkSpawnTile(_) => {
                if highlights.is_none() {
                    create_all_spawn_spot_markers(commands, map)
                }
            }
            MapEditorTool::RemoveSpawnTile => {
                if highlights.is_none() {
                    create_all_spawn_spot_markers(commands, map)
                }
            }
            _ => {
                if highlights.is_some() {
                    remove_all_spawn_spot_markers(commands)
                }
            }
        },
    }
}

fn create_all_spawn_spot_markers(mut commands: Commands, map: Res<GameMap>) {
    let mut tiles = Vec::new();

    for (hex, data) in &map.tiles {
        let Some(_team) = data.spawn_zone else {
            continue;
        };

        tiles.push(hex.clone());
    }

    commands.insert_resource(RangeHighlights { tiles });
}

fn remove_all_spawn_spot_markers(mut commands: Commands) {
    commands.remove_resource::<RangeHighlights>()
}

#[derive(Event)]
pub struct AddSpawnMarkerEvent {
    pub tile: Hex,
    pub team: u8,
}

#[derive(Event)]
pub struct RemoveSpawnMarkerEvent {
    pub tile: Hex,
}

pub fn on_add_spawn_marker(
    mut events: EventReader<AddSpawnMarkerEvent>,
    mut existing_highlights: ResMut<RangeHighlights>,
) {
    for event in events.read() {
        existing_highlights.tiles.push(event.tile);
    }
}

pub fn on_remove_spawn_marker(
    mut events: EventReader<RemoveSpawnMarkerEvent>,
    mut existing_highlights: ResMut<RangeHighlights>,
) {
    for event in events.read() {
        if let Some(pos) = existing_highlights
            .tiles
            .iter()
            .position(|x| x == &event.tile)
        {
            existing_highlights.tiles.swap_remove(pos);
        }
    }
}
