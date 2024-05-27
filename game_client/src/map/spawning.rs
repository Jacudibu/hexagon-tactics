use crate::load::{HexagonMaterials, HexagonMeshes};
use crate::map::map_plugin::{
    HexagonTileComponent, MapState, MapTileEntities, MapTileEntityBundle,
};
use crate::map::tile_cursor::TileRaycastSet;
use crate::map::METERS_PER_TILE_HEIGHT_UNIT;
use bevy::app::{App, Last, Plugin, Startup};
use bevy::core::Name;
use bevy::hierarchy::{BuildChildren, DespawnRecursiveExt};
use bevy::math::{EulerRot, Quat};
use bevy::pbr::{
    AmbientLight, DirectionalLight, DirectionalLightBundle, NotShadowCaster, PbrBundle,
};
use bevy::prelude::{
    default, on_event, Color, Commands, Entity, Event, IntoSystemConfigs, NextState, OnEnter, Res,
    ResMut, SpatialBundle, Transform, Update,
};
use bevy::utils::HashMap;
use bevy_mod_raycast::deferred::RaycastMesh;
use game_common::game_map::{Fluid, GameMap, TileData, HEX_LAYOUT};
use hexx::Hex;

/// Handles (de-)spawning maps based on Events.
/// The tiles for a new map will be spawned whenever `SpawnMapCommand` is sent. Make sure that the map itself has already been added as a resource.
/// In turn, everything will be despawned whenever `DespawnMapCommand` is sent.
pub struct MapSpawningPlugin;
impl Plugin for MapSpawningPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_light);
        app.add_systems(
            Last,
            (
                spawn_map_command_listener.run_if(on_event::<SpawnMapCommand>()),
                despawn_map_command_listener.run_if(on_event::<DespawnMapCommand>()),
            ),
        );

        app.add_systems(OnEnter(MapState::Despawning), despawn_map);
        app.add_systems(OnEnter(MapState::Spawning), spawn_map);

        app.add_event::<SpawnMapCommand>();
        app.add_event::<DespawnMapCommand>();
    }
}

#[derive(Event)]
pub struct SpawnMapCommand {}

#[derive(Event)]
pub struct DespawnMapCommand {}

fn setup_light(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        brightness: 500.0,
        color: Color::ANTIQUE_WHITE,
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 7500.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, 5.7, 0.3, 0.0)),
        ..default()
    });
}

pub fn despawn_map_command_listener(mut new_map_state: ResMut<NextState<MapState>>) {
    new_map_state.set(MapState::Despawning);
}

pub fn despawn_map(
    mut commands: Commands,
    map_entities: ResMut<MapTileEntities>,
    mut new_map_state: ResMut<NextState<MapState>>,
) {
    commands.entity(map_entities.parent).despawn_recursive();
    commands.remove_resource::<MapTileEntities>();
    commands.remove_resource::<GameMap>();

    new_map_state.set(MapState::Unloaded);
}

pub fn spawn_map_command_listener(mut new_map_state: ResMut<NextState<MapState>>) {
    // TODO: Extra step where we load the map resource here.
    new_map_state.set(MapState::Spawning);
}

pub fn spawn_map(
    mut commands: Commands,
    map: Res<GameMap>,
    existing_map_tile_entities: Option<Res<MapTileEntities>>,
    materials: Res<HexagonMaterials>,
    meshes: Res<HexagonMeshes>,
    mut new_map_state: ResMut<NextState<MapState>>,
) {
    if let Some(map_tile_entities) = existing_map_tile_entities {
        commands
            .entity(map_tile_entities.parent)
            .despawn_recursive()
    }

    let map_parent = commands
        .spawn((SpatialBundle::default(), Name::new("Map")))
        .id();

    let mut entities = HashMap::new();
    for (hex, tile_data) in &map.tiles {
        let hex = hex.clone();
        let pos = HEX_LAYOUT.hex_to_world_pos(hex);

        let parent = commands
            .spawn((
                SpatialBundle {
                    transform: Transform::from_xyz(pos.x, 0.0, pos.y),
                    ..default()
                },
                Name::new(format!("Hexagon Tile [{},{}]", hex.x, hex.y)),
            ))
            .set_parent(map_parent)
            .id();

        let top = commands
            .spawn((
                PbrBundle {
                    transform: Transform::from_xyz(
                        0.0,
                        tile_data.height as f32 * METERS_PER_TILE_HEIGHT_UNIT,
                        0.0,
                    ),
                    mesh: meshes.flat.clone(),
                    material: materials.top.surface_material(&tile_data),
                    ..default()
                },
                RaycastMesh::<TileRaycastSet>::default(),
                HexagonTileComponent { hex },
                Name::new(format!("Tile Top [{},{}]", hex.x, hex.y)),
            ))
            .set_parent(parent)
            .id();

        let side = if are_tile_sides_necessary(&map, tile_data, &hex) {
            spawn_side_entity(&mut commands, &materials, &meshes, &tile_data, hex, parent)
        } else {
            None
        };

        let fluid = if let Some(fluid) = &tile_data.fluid {
            spawn_fluid_entity(
                &mut commands,
                &materials,
                &meshes,
                tile_data,
                hex,
                parent,
                fluid,
            )
        } else {
            None
        };

        entities.insert(
            hex,
            MapTileEntityBundle {
                parent,
                top,
                side,
                fluid,
            },
        );
    }

    commands.insert_resource(MapTileEntities {
        parent: map_parent,
        entities,
    });
    new_map_state.set(MapState::Ready);
}

pub fn spawn_side_entity(
    commands: &mut Commands,
    materials: &HexagonMaterials,
    meshes: &HexagonMeshes,
    tile_data: &TileData,
    hex: Hex,
    parent: Entity,
) -> Option<Entity> {
    Some(
        commands
            .spawn((
                PbrBundle {
                    mesh: meshes
                        .columns
                        .get(&tile_data.height)
                        .expect("Meshes for all heights should exist!")
                        .clone(),
                    material: materials.sides.surface_material(&tile_data),
                    ..default()
                },
                RaycastMesh::<TileRaycastSet>::default(),
                HexagonTileComponent { hex },
                Name::new(format!("Tile Side [{},{}]", hex.x, hex.y)),
            ))
            .set_parent(parent)
            .id(),
    )
}

pub fn are_tile_sides_necessary(map: &GameMap, tile_data: &TileData, hex: &Hex) -> bool {
    if tile_data.height == 0 {
        return false;
    }

    hex.all_neighbors().iter().any(|x| match map.tiles.get(x) {
        None => true,
        Some(neighbor) => neighbor.height < tile_data.height,
    })
}

pub fn spawn_fluid_entity(
    commands: &mut Commands,
    materials: &HexagonMaterials,
    meshes: &HexagonMeshes,
    tile_data: &TileData,
    hex: Hex,
    parent: Entity,
    fluid: &Fluid,
) -> Option<Entity> {
    Some(
        commands
            .spawn((
                PbrBundle {
                    transform: Transform::from_xyz(
                        0.0,
                        (tile_data.height as f32 + fluid.height) * METERS_PER_TILE_HEIGHT_UNIT,
                        0.0,
                    ),
                    mesh: meshes.flat.clone(),
                    material: materials.fluid.surface_material(&fluid.kind),
                    ..default()
                },
                RaycastMesh::<TileRaycastSet>::default(),
                NotShadowCaster,
                HexagonTileComponent { hex },
                Name::new(format!("Tile Fluid [{},{}]", hex.x, hex.y)),
            ))
            .set_parent(parent)
            .id(),
    )
}
