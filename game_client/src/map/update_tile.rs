use bevy::app::{App, Plugin, Update};
use bevy::hierarchy::BuildChildren;
use bevy::log::error;
use bevy::math::Vec3;
use bevy::prelude::{
    on_event, Commands, Event, EventReader, IntoSystemConfigs, Query, Res, ResMut, Transform, With,
};
use hexx::Hex;

use game_common::game_map::{GameMap, TileData};

use crate::load::{HexagonMaterials, HexagonMeshes};
use crate::map::spawning::{are_tile_sides_necessary, spawn_fluid_entity, spawn_side_entity};
use crate::map::{HexagonTileComponent, MapTileEntityBundle};
use crate::map::{MapTileEntities, METERS_PER_TILE_HEIGHT_UNIT};

/// Listens to `TileChangeEvent`s in order to push those changes into the existing tile entities.
pub struct TileUpdaterPlugin;
impl Plugin for TileUpdaterPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TileChangeEvent>();
        app.add_systems(
            Update,
            update_tile_entity.run_if(on_event::<TileChangeEvent>()),
        );
    }
}

#[derive(Event)]
pub struct TileChangeEvent {
    pub hex: Hex,
    pub old_data: TileData,
}

pub fn update_tile_entity(
    mut commands: Commands,
    map: Res<GameMap>,
    mut tile_change_event: EventReader<TileChangeEvent>,
    meshes: Res<HexagonMeshes>,
    materials: Res<HexagonMaterials>,
    mut tile_entities: ResMut<MapTileEntities>,
    mut tile_transforms: Query<&mut Transform, With<HexagonTileComponent>>,
) {
    for event in tile_change_event.read() {
        if let Some(tile_data) = map.tiles.get(&event.hex) {
            if let Some(entities) = tile_entities.entities.get_mut(&event.hex) {
                update_sides(
                    &mut commands,
                    &map,
                    &meshes,
                    &materials,
                    &event.hex,
                    tile_data,
                    entities,
                );

                update_top(
                    &mut commands,
                    &materials,
                    &mut tile_transforms,
                    &event.hex,
                    tile_data,
                    entities,
                );

                update_fluid(
                    &mut commands,
                    &meshes,
                    &materials,
                    &mut tile_transforms,
                    &event.hex,
                    tile_data,
                    entities,
                );

                // TODO: Neighbor updates should be triggered separately to avoid updating tiles multiple times when editing an area instead of a single tile... if that ever becomes an option.
                for neighbor in event.hex.all_neighbors() {
                    if let Some(neighbor_data) = map.tiles.get(&neighbor) {
                        if let Some(neighbor_entities) = tile_entities.entities.get_mut(&neighbor) {
                            update_sides(
                                &mut commands,
                                &map,
                                &meshes,
                                &materials,
                                &neighbor,
                                neighbor_data,
                                neighbor_entities,
                            );
                        }
                    }
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

fn update_sides(
    mut commands: &mut Commands,
    map: &Res<GameMap>,
    meshes: &HexagonMeshes,
    materials: &HexagonMaterials,
    hex: &Hex,
    tile_data: &TileData,
    entities: &mut MapTileEntityBundle,
) {
    if let Some(side) = entities.side {
        // Sides still necessary?
        let mut side_commands = commands.entity(side);
        if are_tile_sides_necessary(&map, tile_data, &hex) {
            if let Some(mesh) = meshes.columns.get(&tile_data.height) {
                side_commands.insert(mesh.clone());
                // FIXME: Temporary fix for https://github.com/bevyengine/bevy/issues/4294 and/or https://github.com/aevyrie/bevy_mod_raycast/issues/42
                side_commands.remove::<bevy::render::primitives::Aabb>();
            } else {
                error!(
                    "Was unable to find hex mesh for height {}!",
                    tile_data.height
                );
            }
            side_commands.insert(materials.sides.surface_material(&tile_data));
        } else {
            side_commands.remove_parent();
            side_commands.despawn();
            entities.side = None;
        }
    } else {
        if are_tile_sides_necessary(&map, tile_data, &hex) {
            entities.side = spawn_side_entity(
                &mut commands,
                &materials,
                &meshes,
                tile_data,
                hex.clone(),
                entities.parent,
            );
        } else {
            // Do nothing
        }
    }
}

fn update_top(
    commands: &mut Commands,
    materials: &HexagonMaterials,
    tile_transforms: &mut Query<&mut Transform, With<HexagonTileComponent>>,
    hex: &Hex,
    tile_data: &TileData,
    entities: &mut MapTileEntityBundle,
) {
    let mut top_commands = commands.entity(entities.top);
    if let Ok(mut transform) = tile_transforms.get_mut(entities.top) {
        transform.translation = Vec3::new(
            0.0,
            tile_data.height as f32 * METERS_PER_TILE_HEIGHT_UNIT,
            0.0,
        );
    } else {
        error!("Unable to find a transform for the hex top at {:?}", hex);
    }

    top_commands.insert(materials.top.surface_material(&tile_data));
}

fn update_fluid(
    mut commands: &mut Commands,
    meshes: &HexagonMeshes,
    materials: &HexagonMaterials,
    tile_transforms: &mut Query<&mut Transform, With<HexagonTileComponent>>,
    hex: &Hex,
    tile_data: &TileData,
    entities: &mut MapTileEntityBundle,
) {
    if let Some(fluid) = &tile_data.fluid {
        if let Some(fluid_entity) = entities.fluid {
            if let Ok(mut transform) = tile_transforms.get_mut(fluid_entity) {
                transform.translation = Vec3::new(
                    0.0,
                    (tile_data.height as f32 + fluid.height) * METERS_PER_TILE_HEIGHT_UNIT,
                    0.0,
                );
            } else {
                error!("Unable to find a transform for the hex fluid at {:?}", hex);
            }
        } else {
            entities.fluid = spawn_fluid_entity(
                &mut commands,
                &materials,
                &meshes,
                &tile_data,
                hex.clone(),
                entities.parent,
                &fluid,
            );
        }
    } else {
        if let Some(fluid_entity) = entities.fluid {
            commands.entity(fluid_entity).remove_parent().despawn();
            entities.fluid = None;
        }
    }
}
