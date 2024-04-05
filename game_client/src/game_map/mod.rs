use bevy::app::{App, Plugin, Startup};
use bevy::math::{EulerRot, Quat};
use bevy::pbr::{AmbientLight, DirectionalLight, DirectionalLightBundle, PbrBundle};
use bevy::prelude::*;
use bevy::utils::HashMap;
use bevy_mod_raycast::prelude::RaycastMesh;
use hexx::Hex;

use game_common::game_map::{GameMap, HEX_LAYOUT};
pub use tile_cursor::TileRaycastSet;

use crate::game_map::map_editor::MapEditorPlugin;
use crate::game_map::map_gizmos::MapGizmosPlugin;
use crate::game_map::map_ui::MapUiPlugin;
use crate::game_map::tile_cursor::TileCursorPlugin;
use crate::load::{HexagonMaterials, HexagonMeshes};

mod map_editor;
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
        app.add_systems(Startup, setup_light);
        app.add_systems(
            First,
            (
                on_map_spawned.run_if(resource_added::<MapTileEntities>),
                on_map_despawned.run_if(resource_removed::<MapTileEntities>()),
            ),
        );
        app.init_state::<MapState>();
    }
}

fn on_map_spawned(mut new_map_state: ResMut<NextState<MapState>>) {
    new_map_state.set(MapState::Loaded);
}

fn on_map_despawned(mut new_map_state: ResMut<NextState<MapState>>) {
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
struct MapTileEntities {
    entities: HashMap<Hex, MapTileEntityBundle>,
}

#[derive(Debug)]
struct MapTileEntityBundle {
    top: Entity,
    side: Entity,
}

#[derive(Debug, Component)]
struct TileCoordinates {
    hex: Hex,
}

fn spawn_map(
    map: &GameMap,
    commands: &mut Commands,
    materials: &HexagonMaterials,
    meshes: &HexagonMeshes,
) {
    let map_parent = commands
        .spawn((SpatialBundle::default(), Name::new("Map")))
        .id();

    let mut entities = HashMap::new();
    for (hex, data) in &map.tiles {
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
                        data.height as f32 * METERS_PER_TILE_HEIGHT_UNIT,
                        0.0,
                    ),
                    mesh: meshes.flat.clone(),
                    material: materials.top.surface_material(&data.surface).clone(),
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
                        .get(&data.height)
                        .expect("Meshes for all heights should exist!")
                        .clone(),
                    material: materials.sides.surface_material(&data.surface).clone(),
                    ..default()
                },
                RaycastMesh::<TileRaycastSet>::default(),
                TileCoordinates { hex },
                Name::new(format!("Tile Side [{},{}]", hex.x, hex.y)),
            ))
            .set_parent(parent)
            .id();

        entities.insert(hex, MapTileEntityBundle { top, side });
    }

    commands.insert_resource(MapTileEntities { entities });
}
