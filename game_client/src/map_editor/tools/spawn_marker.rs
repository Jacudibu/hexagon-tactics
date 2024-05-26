use crate::map::{MapState, RangeHighlights};
use crate::map_editor::map_editor_tool::MapEditorTool;
use crate::ApplicationState;
use bevy::app::{App, Plugin};
use bevy::prelude::{
    in_state, on_event, resource_changed_or_removed, Commands, Event, EventReader,
    IntoSystemConfigs, Res, ResMut, Update,
};
use game_common::game_map::GameMap;
use hexx::Hex;

pub struct SpawnMarkerToolPlugin;
impl Plugin for SpawnMarkerToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AddSpawnMarkerEvent>();
        app.add_event::<RemoveSpawnMarkerEvent>();

        app.add_systems(
            Update,
            (
                on_tool_change.run_if(resource_changed_or_removed::<MapEditorTool>()),
                on_add_spawn_marker.run_if(on_event::<AddSpawnMarkerEvent>()),
                on_remove_spawn_marker.run_if(on_event::<RemoveSpawnMarkerEvent>()),
            )
                .run_if(in_state(ApplicationState::MapEditor))
                .run_if(in_state(MapState::Loaded)),
        );
    }
}

#[derive(Event)]
pub struct AddSpawnMarkerEvent {
    pub hex: Hex,
    pub team: u8,
}

#[derive(Event)]
pub struct RemoveSpawnMarkerEvent {
    pub hex: Hex,
}

fn on_tool_change(
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

pub fn on_add_spawn_marker(
    mut events: EventReader<AddSpawnMarkerEvent>,
    mut existing_highlights: ResMut<RangeHighlights>,
) {
    for event in events.read() {
        existing_highlights.tiles.push(event.hex);
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
            .position(|x| x == &event.hex)
        {
            existing_highlights.tiles.swap_remove(pos);
        }
    }
}
