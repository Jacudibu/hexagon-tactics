use bevy::app::{App, First, Plugin};
use bevy::log::error;
use bevy::prelude::*;
use bevy_mod_raycast::deferred::{DeferredRaycastingPlugin, RaycastMesh, RaycastSource};
use bevy_mod_raycast::prelude::RaycastPluginState;
use hexx::Hex;

use crate::camera::MainCamera;
use game_common::game_map::{GameMap, HEX_LAYOUT};

use crate::map::map_plugin::{HexagonTileComponent, MapState};
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
            (update_mouse_cursor
                .after(bevy_mod_raycast::deferred::update_target_intersections::<TileRaycastSet>))
            .run_if(in_state(MouseCursorOverUiState::NotOverUI))
            .run_if(in_state(MapState::Loaded)),
        );
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

#[derive(Reflect)]
pub struct TileRaycastSet;

#[derive(Component, Debug)]
pub struct TileCursor {
    pub hex: Hex,
}

#[derive(Resource, Debug)]
pub struct MouseCursorOnTile {
    pub temp_hexes: Vec<Hex>, // TODO: remove this
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
                                mouse_cursor.temp_hexes = vec![tile_coordinates.hex];
                            }
                        } else {
                            commands.insert_resource(MouseCursorOnTile {
                                hex: tile_coordinates.hex,
                                temp_hexes: vec![tile_coordinates.hex],
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
