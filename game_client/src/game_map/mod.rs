use bevy::app::{App, Plugin, Startup};
use bevy::math::{EulerRot, Quat};
use bevy::pbr::{AmbientLight, DirectionalLight, DirectionalLightBundle, PbrBundle};
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_mod_raycast::prelude::RaycastMesh;
use hexx::Hex;

use game_common::game_map::{GameMap, HEX_LAYOUT};
pub use tile_cursor::TileRaycastSet;

use crate::game_map::editor::MapEditorPlugin;
use crate::game_map::map_gizmos::MapGizmosPlugin;
use crate::game_map::map_ui::MapUiPlugin;
use crate::game_map::tile_cursor::TileCursorPlugin;
use crate::load::{HexagonMaterials, HexagonMeshes};

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
                setup_debug_map
                    // TODO: Use Asset Loader instead of making these pub.
                    .after(crate::load::load_meshes)
                    .after(crate::load::load_materials),
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
    commands: &mut Commands,
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
