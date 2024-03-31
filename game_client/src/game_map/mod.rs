use crate::game_map::map_gizmos::MapGizmosPlugin;
use crate::game_map::tile_cursor::{TileCursorPlugin, TileRaycastSet};
use bevy::app::{App, Plugin, Startup};
use bevy::asset::{Assets, Handle};
use bevy::math::{EulerRot, Quat, Vec2, Vec3};
use bevy::pbr::{
    AmbientLight, DirectionalLight, DirectionalLightBundle, PbrBundle, StandardMaterial,
};
use bevy::prelude::*;
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::utils::HashMap;
use bevy_basic_camera::CameraController;
use bevy_mod_raycast::deferred::RaycastSource;
use bevy_mod_raycast::prelude::RaycastMesh;
use game_common::game_map;
use game_common::game_map::GameMap;
use hexx::{ColumnMeshBuilder, Hex, HexLayout, HexOrientation};

mod editor;
mod map_gizmos;
mod tile_cursor;

pub struct GameMapPlugin;
impl Plugin for GameMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TileCursorPlugin);
        app.add_plugins(MapGizmosPlugin);
        app.add_systems(
            Startup,
            (
                load_meshes,
                setup_camera,
                setup_grid.after(load_meshes),
                setup_light,
            ),
        );
    }
}

pub const METERS_PER_TILE_HEIGHT_UNIT: f32 = 0.5;

fn setup_camera(mut commands: Commands) {
    let transform = Transform::from_xyz(0.0, 60.0, 60.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands.spawn((
        Camera3dBundle {
            transform,
            ..default()
        },
        RaycastSource::<TileRaycastSet>::new_cursor(),
        CameraController {
            sensitivity: 2.0,
            walk_speed: 20.0,
            run_speed: 50.0,
            ..default()
        },
    ));
}

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
    entities: HashMap<Hex, Entity>,
}

#[derive(Debug, Resource)]
struct MapMaterials {
    highlighted_material: Handle<StandardMaterial>,
    default_material: Handle<StandardMaterial>,
}

#[derive(Debug, Component)]
struct TileCoordinates {
    hex: Hex,
    height: u8,
}

#[derive(Debug, Resource)]
pub struct HexagonMeshes {
    flat: Handle<Mesh>,
    columns: HashMap<u8, Handle<Mesh>>,
}

// TODO: Move this into some asset_loader-style loading state
fn load_meshes(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    // TODO: Layout is probably best stores as a resource as well
    let layout = HexLayout {
        origin: Vec2::ZERO,
        hex_size: Vec2::splat(1.0),
        orientation: HexOrientation::Pointy,
        ..default()
    };

    let flat_mesh = generate_hexagonal_column_mesh(&layout, 0.0);
    let flat = meshes.add(flat_mesh);

    let mut columns = HashMap::new();
    for height in game_map::MIN_HEIGHT..=game_map::MAX_HEIGHT {
        let mesh =
            generate_hexagonal_column_mesh(&layout, height as f32 * METERS_PER_TILE_HEIGHT_UNIT);
        let handle = meshes.add(mesh);
        columns.insert(height, handle);
    }

    commands.insert_resource(HexagonMeshes { flat, columns })
}

fn setup_grid(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    meshes: Res<HexagonMeshes>,
) {
    let radius = 20;
    let map = GameMap::new(radius);

    let default_material = materials.add(Color::WHITE);
    let highlighted_material = materials.add(Color::YELLOW);

    let mut entities = HashMap::new();
    for (hex, data) in &map.tiles {
        let hex = hex.clone();
        let pos = map.layout.hex_to_world_pos(hex);
        let id = commands
            .spawn((
                PbrBundle {
                    transform: Transform::from_xyz(pos.x, 0.0, pos.y),
                    mesh: meshes
                        .columns
                        .get(&data.height)
                        .expect("Meshes for all heights should exist!")
                        .clone(),
                    material: default_material.clone(),
                    ..default()
                },
                RaycastMesh::<TileRaycastSet>::default(),
                TileCoordinates {
                    hex,
                    height: data.height,
                },
            ))
            .id();

        entities.insert(hex, id);
    }

    commands.insert_resource(map);
    commands.insert_resource(MapTileEntities { entities });
    commands.insert_resource(MapMaterials {
        highlighted_material,
        default_material,
    });
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
