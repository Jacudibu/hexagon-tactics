use crate::game_map::tile_cursor::{TileCursorPlugin, TileRaycastSet};
use bevy::app::{App, Plugin, Startup, Update};
use bevy::asset::{Assets, Handle};
use bevy::math::{EulerRot, Quat, Vec3};
use bevy::pbr::{
    AmbientLight, DirectionalLight, DirectionalLightBundle, PbrBundle, StandardMaterial,
};
use bevy::prelude::{
    default, AppGizmoBuilder, Camera3dBundle, Color, Commands, Component, Entity, GizmoConfigGroup,
    GizmoConfigStore, Gizmos, Mesh, Reflect, Res, ResMut, Resource, Transform,
};
use bevy::render::mesh::{Indices, PrimitiveTopology};
use bevy::render::render_asset::RenderAssetUsages;
use bevy::utils::HashMap;
use bevy_basic_camera::CameraController;
use bevy_mod_raycast::prelude::RaycastMesh;
use game_common::game_map;
use game_common::game_map::GameMap;
use hexx::{ColumnMeshBuilder, GridVertex, Hex, HexLayout};

mod editor;
mod tile_cursor;

pub struct GameMapPlugin;
impl Plugin for GameMapPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<MapGizmos>();
        app.add_plugins(TileCursorPlugin);
        app.add_systems(
            Startup,
            (setup_camera, setup_grid, set_gizmo_config, setup_light),
        );
        app.add_systems(Update, draw_hexagon_gizmos);
    }
}

pub const METERS_PER_TILE_HEIGHT_UNIT: f32 = 0.5;

fn setup_camera(mut commands: Commands) {
    let transform = Transform::from_xyz(0.0, 60.0, 60.0).looking_at(Vec3::ZERO, Vec3::Y);
    commands
        .spawn(Camera3dBundle {
            transform,
            ..default()
        })
        .insert(CameraController {
            sensitivity: 2.0,
            walk_speed: 20.0,
            run_speed: 50.0,
            ..default()
        });
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

fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let radius = 20;
    let map = GameMap::new(radius);

    let default_material = materials.add(Color::WHITE);
    let highlighted_material = materials.add(Color::YELLOW);

    let mut mesh_handles = HashMap::new();
    for height in game_map::MIN_HEIGHT..=game_map::MAX_HEIGHT {
        let mesh = generate_hexagonal_column_mesh(
            &map.layout,
            height as f32 * METERS_PER_TILE_HEIGHT_UNIT,
        );
        let handle = meshes.add(mesh);
        mesh_handles.insert(height, handle);
    }

    let mut entities = HashMap::new();
    for (hex, data) in &map.tiles {
        let hex = hex.clone();
        let pos = map.layout.hex_to_world_pos(hex);
        let id = commands
            .spawn((
                PbrBundle {
                    transform: Transform::from_xyz(pos.x, 0.0, pos.y),
                    mesh: mesh_handles
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
        RenderAssetUsages::RENDER_WORLD,
    )
    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, mesh_info.vertices)
    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_info.normals)
    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, mesh_info.uvs)
    .with_inserted_indices(Indices::U16(mesh_info.indices))
}

#[derive(Default, Reflect, GizmoConfigGroup)]
struct MapGizmos;

fn draw_hexagon_gizmos(mut gizmos: Gizmos<MapGizmos>, map: Res<GameMap>) {
    for (hex, data) in &map.tiles {
        let height = data.height as f32 * METERS_PER_TILE_HEIGHT_UNIT;
        let top_vertices = hex
            .all_vertices()
            .map(|x| vertex_coordinates_3d(&map.layout, x, height));

        connect_hexagon_vertices(&mut gizmos, top_vertices);

        // for mid_height in 1..data.height {
        //     let mid_height = mid_height as f32 * METERS_PER_TILE_HEIGHT_UNIT;
        //   //   let vertices = hex
        //         .all_vertices()
        //         .map(|x| vertex_coordinates_3d(&map.layout, x, mid_height));
        //     connect_hexagon_vertices(&mut gizmos, vertices);
        // }

        let bottom_vertices = hex
            .all_vertices()
            .map(|x| vertex_coordinates_3d(&map.layout, x, 0.0));

        gizmos.line(top_vertices[0], bottom_vertices[0], Color::BLACK);
        gizmos.line(top_vertices[1], bottom_vertices[1], Color::BLACK);
        gizmos.line(top_vertices[2], bottom_vertices[2], Color::BLACK);
        gizmos.line(top_vertices[3], bottom_vertices[3], Color::BLACK);
        gizmos.line(top_vertices[4], bottom_vertices[4], Color::BLACK);
        gizmos.line(top_vertices[5], bottom_vertices[5], Color::BLACK);
    }
}

fn connect_hexagon_vertices(gizmos: &mut Gizmos<MapGizmos>, vertices: [Vec3; 6]) {
    gizmos.line(vertices[0], vertices[1], Color::BLACK);
    gizmos.line(vertices[1], vertices[2], Color::BLACK);
    gizmos.line(vertices[2], vertices[3], Color::BLACK);
    gizmos.line(vertices[3], vertices[4], Color::BLACK);
    gizmos.line(vertices[4], vertices[5], Color::BLACK);
    gizmos.line(vertices[5], vertices[0], Color::BLACK);
}

#[must_use]
pub fn vertex_coordinates_3d(layout: &HexLayout, vertex: GridVertex, height: f32) -> Vec3 {
    let vertex_coordinates = layout.vertex_coordinates(vertex);
    Vec3 {
        x: vertex_coordinates.x,
        y: height,
        z: vertex_coordinates.y,
    }
}

fn set_gizmo_config(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<MapGizmos>();
    config.depth_bias = -0.00001;
    config.line_width = 20.0;
    config.line_perspective = true;
}