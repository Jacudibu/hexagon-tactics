use crate::load::{CursorMaterials, HexagonMeshes};
use crate::map::tile_cursor::position_for_tile;
use bevy::core::Name;
use bevy::pbr::{NotShadowCaster, PbrBundle};
use bevy::prelude::{default, Commands, Component, Entity, Query, Res, Resource, Transform, With};
use game_common::game_map::GameMap;
use hexx::Hex;

#[derive(Debug, Resource)]
pub struct HighlightedTiles {
    pub tiles: Vec<Hex>,
}

#[derive(Component)]
pub struct HighlightedTileMarker;

const EXTRA_HEIGHT: f32 = 0.005;

pub fn on_highlight_change(
    mut commands: Commands,
    existing_highlights: Query<Entity, With<HighlightedTileMarker>>,
    highlighted_tiles: Option<Res<HighlightedTiles>>,
    map: Res<GameMap>,
    hexagon_meshes: Res<HexagonMeshes>,
    cursor_materials: Res<CursorMaterials>,
) {
    // TODO: Would probably be cheaper to move highlights instead of respawning them :^)
    for entity in existing_highlights.iter() {
        commands.entity(entity).despawn();
    }

    let Some(highlighted_tiles) = highlighted_tiles else {
        return;
    };

    for hex in highlighted_tiles.tiles.iter() {
        let translation = position_for_tile(&map, hex, EXTRA_HEIGHT);

        commands.spawn((
            Name::new(format!("Highlight [{},{}]", hex.x, hex.y)),
            HighlightedTileMarker,
            PbrBundle {
                mesh: hexagon_meshes.flat.clone(),
                transform: Transform::from_translation(translation),
                material: cursor_materials.default.clone(),
                ..default()
            },
            NotShadowCaster,
        ));
    }
}
