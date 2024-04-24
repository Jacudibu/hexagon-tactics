use crate::load::{CursorMaterials, HexagonMeshes};
use crate::map::highlights::range_highlights::RangeHighlightMarker;
use crate::map::highlights::HighlightedTiles;
use crate::map::tile_cursor::position_for_tile;
use crate::map::RangeHighlights;
use bevy::app::{App, Update};
use bevy::core::Name;
use bevy::pbr::{NotShadowCaster, PbrBundle};
use bevy::prelude::{
    default, resource_changed_or_removed, Commands, Component, Entity, IntoSystemConfigs, Plugin,
    Query, Res, Resource, Transform, With,
};
use game_common::game_map::GameMap;

const EXTRA_HEIGHT: f32 = 0.005;

pub struct HighlightPlugin;
impl Plugin for HighlightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            on_highlight_change::<RangeHighlightMarker, RangeHighlights>
                .run_if(resource_changed_or_removed::<RangeHighlights>()),
        );
    }
}

fn on_highlight_change<TMarker: Component + Default, TResource: Resource + HighlightedTiles>(
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
