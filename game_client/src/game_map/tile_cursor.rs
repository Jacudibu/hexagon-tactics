use crate::game_map::TileCoordinates;
use crate::MouseCursorOverUiState;
use bevy::app::{App, First, Plugin};
use bevy::core::Name;
use bevy::hierarchy::Parent;
use bevy::log::error;
use bevy::math::{IVec2, Quat};
use bevy::pbr::NotShadowCaster;
use bevy::prelude::{
    default, in_state, Commands, Component, Entity, IntoSystemConfigs, Query, Reflect, Res,
    Resource, Transform, Vec3, With,
};
use bevy_mod_raycast::deferred::{RaycastMesh, RaycastSource};
use hexx::Hex;

pub(in crate::game_map) struct TileCursorPlugin;
impl Plugin for TileCursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            First,
            (
                update_mouse_cursor.run_if(in_state(MouseCursorOverUiState::NotOverUI)),
                update_tile_cursor
                    .after(update_mouse_cursor)
                    .run_if(in_state(MouseCursorOverUiState::NotOverUI)),
            ),
        );
    }
}

#[derive(Reflect)]
pub struct TileRaycastSet;

#[derive(Component, Debug)]
pub struct TileCursor {
    pub hex: Hex,
}

#[derive(Resource, Debug)]
pub struct MouseCursorOnTile {
    pub hex: Hex,
    pub world_pos: Vec3,
}

fn update_mouse_cursor(
    mut commands: Commands,
    tile_ray: Query<&RaycastSource<TileRaycastSet>>,
    ray_targets: Query<&TileCoordinates, With<RaycastMesh<TileRaycastSet>>>,
) {
    for source in tile_ray.iter() {
        if let Some(intersections) = source.get_intersections() {
            for (entity, intersection) in intersections {
                match ray_targets.get(entity.clone()) {
                    Ok(tile_coordinates) => {
                        commands.insert_resource(MouseCursorOnTile {
                            hex: tile_coordinates.hex,
                            world_pos: intersection.position(),
                        });
                        return;
                    }
                    Err(e) => {
                        error!("Unexpected error when Raycasting for mouse cursor: {}", e)
                    }
                }
            }
        }
    }

    commands.remove_resource::<MouseCursorOnTile>()
}

fn update_tile_cursor(
    mut commands: Commands,
    mouse_cursor: Option<Res<MouseCursorOnTile>>,
    tile_cursor_q: Query<(Entity, &TileCursor)>,
) {
    let Some(mouse_cursor) = mouse_cursor else {
        return;
    };

    let this_frame_selection: Vec<Hex> = vec![mouse_cursor.hex];

    let mut already_existing_cursors: Vec<Hex> = Vec::new();
    for (entity, cursor) in tile_cursor_q.iter() {
        if this_frame_selection.iter().any(|pos| pos == &cursor.hex) {
            already_existing_cursors.push(cursor.hex);
        } else {
            commands.entity(entity).despawn();
        }
    }

    for selected_tile in this_frame_selection.iter() {
        if !already_existing_cursors.contains(selected_tile) {
            commands.spawn((
                Name::new(format!(
                    "Tile Cursor [{},{}]",
                    selected_tile.x, selected_tile.y
                )),
                TileCursor {
                    hex: selected_tile.clone(),
                },
                // TODO: Graphical Representation. A flat hex mesh should be enough.
                NotShadowCaster,
            ));
        }
    }
}
