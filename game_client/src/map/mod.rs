use bevy::app::{App, Plugin, Startup};
use bevy::math::{EulerRot, Quat};
use bevy::pbr::{
    AmbientLight, DirectionalLight, DirectionalLightBundle, NotShadowCaster, PbrBundle,
};
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_mod_raycast::prelude::RaycastMesh;
use hexx::Hex;

use game_common::game_map::{Fluid, GameMap, TileData, HEX_LAYOUT};
pub use tile_cursor::TileRaycastSet;

use crate::load::{HexagonMaterials, HexagonMeshes};
use crate::map::map_gizmos::MapGizmosPlugin;
use crate::map::map_ui::MapUiPlugin;
use crate::map::tile_cursor::TileCursorPlugin;

mod map_gizmos;
mod map_ui;
mod tile_cursor;
pub use tile_cursor::TileCursor;

pub struct GameMapPlugin;
impl Plugin for GameMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TileCursorPlugin);
        app.add_plugins(MapGizmosPlugin);
        app.add_plugins(MapUiPlugin);
        app.add_systems(Startup, setup_light);
        app.add_systems(
            First,
            (
                on_map_spawned.run_if(resource_added::<MapTileEntities>),
                on_map_despawned.run_if(resource_removed::<MapTileEntities>()),
                spawn_map_command_listener.run_if(on_event::<SpawnMapCommand>()),
            ),
        );
        app.init_state::<MapState>();
        app.add_event::<SpawnMapCommand>();
    }
}

fn on_map_spawned(mut new_map_state: ResMut<NextState<MapState>>) {
    new_map_state.set(MapState::Loaded);
}

fn on_map_despawned(mut commands: Commands, mut new_map_state: ResMut<NextState<MapState>>) {
    commands.remove_resource::<GameMap>();
    new_map_state.set(MapState::Unloaded);
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Reflect)]
pub enum MapState {
    #[default]
    Unloaded,
    Loaded,
}

pub const METERS_PER_TILE_HEIGHT_UNIT: f32 = 0.5;

fn setup_light(mut commands: Commands) {
    commands.insert_resource(AmbientLight {
        brightness: 20.0,
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 1000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, 5.7, 0.3, 0.0)),
        ..default()
    });
}

#[derive(Debug, Resource)]
pub struct MapTileEntities {
    pub parent: Entity,
    pub entities: HashMap<Hex, MapTileEntityBundle>,
}

#[derive(Debug)]
pub struct MapTileEntityBundle {
    pub parent: Entity,
    pub top: Entity,
    pub side: Entity,
    pub fluid: Option<Entity>,
}

#[derive(Debug, Component)]
pub struct TileCoordinates {
    hex: Hex,
}

#[derive(Event)]
pub struct SpawnMapCommand {}

fn spawn_map_command_listener(
    mut commands: Commands,
    map: Res<GameMap>,
    existing_map_tile_entities: Option<Res<MapTileEntities>>,
    materials: Res<HexagonMaterials>,
    meshes: Res<HexagonMeshes>,
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
                TileCoordinates { hex },
                Name::new(format!("Tile Top [{},{}]", hex.x, hex.y)),
            ))
            .set_parent(parent)
            .id();

        let side = commands
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
                TileCoordinates { hex },
                Name::new(format!("Tile Side [{},{}]", hex.x, hex.y)),
            ))
            .set_parent(parent)
            .id();

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
                TileCoordinates { hex },
                Name::new(format!("Tile Fluid [{},{}]", hex.x, hex.y)),
            ))
            .set_parent(parent)
            .id(),
    )
}
