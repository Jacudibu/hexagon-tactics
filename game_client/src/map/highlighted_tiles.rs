use crate::load::{CursorMaterials, HexagonMeshes};
use crate::map::tile_cursor::position_for_tile;
use bevy::asset::Handle;
use bevy::core::Name;
use bevy::pbr::{NotShadowCaster, PbrBundle, StandardMaterial};
use bevy::prelude::{default, Commands, Component, Entity, Query, Res, Resource, Transform, With};
use game_common::game_map::GameMap;
use hexx::Hex;

pub trait HighlightedTiles {
    fn tiles(&self) -> &Vec<Hex>;
    fn material(materials: &CursorMaterials) -> Handle<StandardMaterial>;
}

#[derive(Debug, Resource)]
pub struct RangeHighlights {
    pub tiles: Vec<Hex>,
}

impl HighlightedTiles for RangeHighlights {
    fn tiles(&self) -> &Vec<Hex> {
        &self.tiles
    }

    fn material(materials: &CursorMaterials) -> Handle<StandardMaterial> {
        materials.range_highlight.clone()
    }
}

#[derive(Component, Default)]
pub struct RangeHighlightMarker;

const EXTRA_HEIGHT: f32 = 0.005;

pub fn on_highlight_change<TMarker: Component + Default, TResource: Resource + HighlightedTiles>(
    mut commands: Commands,
    existing_highlights: Query<Entity, With<TMarker>>,
    highlighted_tiles: Option<Res<TResource>>,
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

    for hex in highlighted_tiles.tiles().iter() {
        let translation = position_for_tile(&map, hex, EXTRA_HEIGHT);

        commands.spawn((
            Name::new(format!("Highlight [{},{}]", hex.x, hex.y)),
            TMarker::default(),
            PbrBundle {
                mesh: hexagon_meshes.flat.clone(),
                transform: Transform::from_translation(translation),
                material: TResource::material(&cursor_materials),
                ..default()
            },
            NotShadowCaster,
        ));
    }
}
