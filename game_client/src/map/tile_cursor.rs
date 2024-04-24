use bevy::app::{App, First, Plugin};
use bevy::core::Name;
use bevy::log::error;
use bevy::pbr::{NotShadowCaster, PbrBundle};
use bevy::prelude::*;
use bevy_mod_raycast::deferred::{DeferredRaycastingPlugin, RaycastMesh, RaycastSource};
use bevy_mod_raycast::prelude::RaycastPluginState;
use hexx::Hex;

use crate::camera::MainCamera;
use game_common::game_map::{GameMap, HEX_LAYOUT};

use crate::load::{CursorMaterials, HexagonMeshes};
use crate::map::map_plugin::{HexagonTileComponent, MapState};
use crate::map::update_tile::TileChangeEvent;
use crate::map::METERS_PER_TILE_HEIGHT_UNIT;
use crate::MouseCursorOverUiState;

pub(in crate::map) struct TileCursorPlugin;
impl Plugin for TileCursorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DeferredRaycastingPlugin::<TileRaycastSet>::default());
        app.insert_resource(RaycastPluginState::<TileRaycastSet>::default());
        app.add_systems(OnEnter(MapState::Loaded), setup_camera);
        app.add_systems(
            First,
            (
                update_mouse_cursor.after(
                    bevy_mod_raycast::deferred::update_target_intersections::<TileRaycastSet>,
                ),
                update_tile_cursor,
            )
                .chain()
                .run_if(in_state(MouseCursorOverUiState::NotOverUI))
                .run_if(in_state(MapState::Loaded)),
        );
        app.add_systems(
            Update,
            handle_tile_change_event.run_if(in_state(MapState::Loaded)),
        );
        app.add_systems(OnExit(MapState::Loaded), clean_up);
    }
}

fn setup_camera(
    mut commands: Commands,
    main_camera: Query<Entity, (With<MainCamera>, Without<RaycastSource<TileRaycastSet>>)>,
) {
    for entity in main_camera.iter() {
        commands
            .entity(entity)
            .insert(RaycastSource::<TileRaycastSet>::new_cursor());
    }
}

fn clean_up(mut commands: Commands, cursors: Query<Entity, With<TileCursor>>) {
    for x in cursors.iter() {
        commands.entity(x).despawn();
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
}

fn update_mouse_cursor(
    mut commands: Commands,
    mouse_cursor: Option<ResMut<MouseCursorOnTile>>,
    tile_ray: Query<&RaycastSource<TileRaycastSet>>,
    ray_targets: Query<&HexagonTileComponent, With<RaycastMesh<TileRaycastSet>>>,
) {
    for source in tile_ray.iter() {
        if let Some(intersections) = source.get_intersections() {
            let nearest = intersections
                .iter()
                .min_by(|(_, a), (_, b)| a.distance().total_cmp(&b.distance()));
            if let Some((entity, _intersection)) = nearest {
                match ray_targets.get(entity.clone()) {
                    Ok(tile_coordinates) => {
                        if let Some(mut mouse_cursor) = mouse_cursor {
                            // Avoid change detection if the tile is still the same
                            if mouse_cursor.hex != tile_coordinates.hex {
                                mouse_cursor.hex = tile_coordinates.hex;
                            }
                        } else {
                            commands.insert_resource(MouseCursorOnTile {
                                hex: tile_coordinates.hex,
                            });
                        }
                        return;
                    }
                    Err(e) => {
                        error!("Unexpected error when Raycasting for mouse cursor: {}", e)
                    }
                }
            }
        }
    }

    if mouse_cursor.is_some() {
        commands.remove_resource::<MouseCursorOnTile>()
    }
}

fn handle_tile_change_event(
    map: Res<GameMap>,
    mut tile_change_event: EventReader<TileChangeEvent>,
    mut tile_cursor_q: Query<(&TileCursor, &mut Transform)>,
) {
    let changed_hexagons: Vec<Hex> = tile_change_event.read().map(|x| x.hex).collect();
    for (cursor, mut transform) in tile_cursor_q.iter_mut() {
        if changed_hexagons.contains(&cursor.hex) {
            transform.translation = position_for_tile(&map, &cursor.hex, EXTRA_HEIGHT)
        }
    }
}

fn update_tile_cursor(
    mut commands: Commands,
    mouse_cursor: Option<Res<MouseCursorOnTile>>,
    tile_cursor_q: Query<(Entity, &TileCursor)>,
    hexagon_meshes: Res<HexagonMeshes>,
    cursor_materials: Res<CursorMaterials>,
    map: Res<GameMap>,
) {
    let Some(mouse_cursor) = mouse_cursor else {
        for (entity, _) in tile_cursor_q.iter() {
            commands.entity(entity).despawn();
        }

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
            let translation = position_for_tile(&map, selected_tile, EXTRA_HEIGHT);

            commands.spawn((
                Name::new(format!(
                    "Tile Cursor [{},{}]",
                    selected_tile.x, selected_tile.y
                )),
                TileCursor {
                    hex: selected_tile.clone(),
                },
                PbrBundle {
                    mesh: hexagon_meshes.flat.clone(),
                    transform: Transform::from_translation(translation),
                    material: cursor_materials.default.clone(),
                    ..default()
                },
                NotShadowCaster,
            ));
        }
    }
}

const EXTRA_HEIGHT: f32 = 0.01;
pub fn position_for_tile(map: &GameMap, hex: &Hex, extra_height: f32) -> Vec3 {
    let position = HEX_LAYOUT.hex_to_world_pos(hex.clone());
    let height = if let Some(tile) = map.tiles.get(hex) {
        if let Some(fluid) = &tile.fluid {
            tile.height as f32 + fluid.height
        } else {
            tile.height as f32
        }
    } else {
        error!("Was unable to find a tile for {:?} in map.", hex);
        0.0
    };

    Vec3 {
        x: position.x,
        y: height * METERS_PER_TILE_HEIGHT_UNIT + extra_height,
        z: position.y,
    }
}
