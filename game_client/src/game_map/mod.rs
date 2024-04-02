use bevy::app::{App, Plugin, Startup};
use bevy::asset::{Assets, Handle};
use bevy::math::{EulerRot, Quat};
use bevy::pbr::{
    AmbientLight, DirectionalLight, DirectionalLightBundle, PbrBundle, StandardMaterial,
};
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::utils::HashMap;
use bevy_mod_raycast::prelude::RaycastMesh;
use hexx::{ColumnMeshBuilder, Hex, HexLayout};

use game_common::game_map;
use game_common::game_map::{GameMap, TileSurface, HEX_LAYOUT};
pub use tile_cursor::TileRaycastSet;

use crate::game_map::editor::MapEditorPlugin;
use crate::game_map::map_gizmos::MapGizmosPlugin;
use crate::game_map::map_ui::MapUiPlugin;
use crate::game_map::tile_cursor::TileCursorPlugin;

mod editor;
mod map_gizmos;
mod map_ui;
mod tile_cursor;

pub struct GameMapPlugin;
impl Plugin for GameMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TileCursorPlugin);
        app.add_plugins(MapGizmosPlugin);
        app.add_plugins(MapEditorPlugin);
        app.add_plugins(MapUiPlugin);
        app.add_systems(
            Startup,
            (
                load_meshes,
                load_materials,
                setup_debug_map.after(load_meshes).after(load_materials),
                setup_light,
            ),
        );
    }
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
struct MapTileEntities {
    parent: Entity,
    entities: HashMap<Hex, Entity>,
}

#[derive(Debug, Component)]
struct TileCoordinates {
    hex: Hex,
    height: u8,
}

#[derive(Debug, Resource)]
struct HexagonMaterials {
    invisible: Handle<StandardMaterial>,
    grass: Handle<StandardMaterial>,
    stone: Handle<StandardMaterial>,
    sand: Handle<StandardMaterial>,
    earth: Handle<StandardMaterial>,
    water: Handle<StandardMaterial>,
}

impl HexagonMaterials {
    #[must_use]
    pub fn surface_material(&self, surface: &TileSurface) -> Handle<StandardMaterial> {
        match surface {
            TileSurface::Grass => self.grass.clone(),
            TileSurface::Stone => self.stone.clone(),
            TileSurface::Sand => self.sand.clone(),
            TileSurface::Earth => self.earth.clone(),
            TileSurface::Water => self.water.clone(),
        }
    }
}

#[derive(Debug, Resource)]
pub struct HexagonMeshes {
    flat: Handle<Mesh>,
    columns: HashMap<u8, Handle<Mesh>>,
}

// TODO: Move this into some asset_loader-style loading state
fn load_meshes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    let flat_mesh = generate_hexagonal_column_mesh(&HEX_LAYOUT, 0.0);
    let flat = meshes.add(flat_mesh);

    let mut columns = HashMap::new();
    for height in 0..=game_map::MAX_HEIGHT {
        let mesh = generate_hexagonal_column_mesh(
            &HEX_LAYOUT,
            height as f32 * METERS_PER_TILE_HEIGHT_UNIT,
        );
        let handle = meshes.add(mesh);
        columns.insert(height, handle);
    }

    commands.insert_resource(HexagonMeshes { flat, columns })
}

fn load_materials(mut commands: Commands, mut materials: ResMut<Assets<StandardMaterial>>) {
    commands.insert_resource(HexagonMaterials {
        invisible: materials.add(Color::rgba(0.0, 0.0, 0.0, 0.0)),
        grass: materials.add(Color::GREEN),
        stone: materials.add(Color::GRAY),
        sand: materials.add(Color::BEIGE),
        earth: materials.add(Color::TOMATO),
        water: materials.add(Color::BLUE),
    });
}

fn setup_debug_map(
    mut commands: Commands,
    materials: Res<HexagonMaterials>,
    meshes: Res<HexagonMeshes>,
) {
    let radius = 20;
    let map = GameMap::new(radius);

    spawn_map(&map, &mut commands, &materials, &meshes);

    commands.insert_resource(map);
}

fn spawn_map(
    map: &GameMap,
    mut commands: &mut Commands,
    materials: &HexagonMaterials,
    meshes: &HexagonMeshes,
) {
    let parent = commands
        .spawn((SpatialBundle::default(), Name::new("Map")))
        .id();

    let mut entities = HashMap::new();
    for (hex, data) in &map.tiles {
        let hex = hex.clone();
        let pos = HEX_LAYOUT.hex_to_world_pos(hex);
        let id = commands
            .spawn((
                PbrBundle {
                    transform: Transform::from_xyz(pos.x, 0.0, pos.y),
                    mesh: meshes
                        .columns
                        .get(&data.height)
                        .expect("Meshes for all heights should exist!")
                        .clone(),
                    material: materials.surface_material(&data.surface).clone(),
                    ..default()
                },
                RaycastMesh::<TileRaycastSet>::default(),
                TileCoordinates {
                    hex,
                    height: data.height,
                },
                Name::new(format!("Hexagon Tile [{},{}]", hex.x, hex.y)),
            ))
            .set_parent(parent)
            .id();

        entities.insert(hex, id);
    }

    commands.insert_resource(MapTileEntities { parent, entities });
}

/// Compute a bevy mesh from the layout
fn generate_hexagonal_column_mesh(hex_layout: &HexLayout, height: f32) -> Mesh {
    let mesh_info = ColumnMeshBuilder::new(hex_layout, height)
        .without_bottom_face()
        .center_aligned()
        .build();
    Mesh::new(
        PrimitiveTopology::TriangleList,
        RenderAssetUsages::default(),
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}
