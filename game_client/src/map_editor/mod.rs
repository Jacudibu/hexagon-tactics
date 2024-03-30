use bevy::app::App;
use bevy::prelude::*;
use bevy::render::mesh::Indices;
use bevy::render::render_asset::RenderAssetUsages;
use bevy::render::render_resource::PrimitiveTopology;
use bevy::utils::HashMap;
use bevy_basic_camera::CameraController;
use game_common::game_map::GameMap;
use hexx::{ColumnMeshBuilder, Hex, HexLayout, HexOrientation, Vec2};

pub struct MapEditorPlugin;
impl Plugin for MapEditorPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(AmbientLight {
            brightness: 2000.0,
            ..default()
        });
        app.add_systems(Startup, (setup_camera, setup_grid));
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

fn setup_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let radius = 20;
    let layout = HexLayout {
        hex_size: Vec2::splat(1.0),
        orientation: HexOrientation::Flat,
        ..default()
    };

    // materials
    let default_material = materials.add(Color::WHITE);
    let highlighted_material = materials.add(Color::YELLOW);
    // mesh
    let mesh = generate_hexagonal_column_mesh(&layout, 1.0);
    let mesh_handle = meshes.add(mesh);

    let map = GameMap::new(radius);

    let mut entities = HashMap::new();
    for (hex, data) in &map.tiles {
        let hex = hex.clone();
        let pos = layout.hex_to_world_pos(hex);
        let id = commands
            .spawn(PbrBundle {
                transform: Transform::from_xyz(
                    pos.x,
                    data.height as f32 * METERS_PER_TILE_HEIGHT_UNIT,
                    pos.y,
                ),
                mesh: mesh_handle.clone(),
                material: default_material.clone(),
                ..default()
            })
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
