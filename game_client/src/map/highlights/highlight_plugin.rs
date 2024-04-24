use crate::load::{CursorMaterials, HexagonMeshes};
use crate::map::highlights::attack_highlights::{AttackHighlightMarker, AttackHighlights};
use crate::map::highlights::cursor_highlights::CursorHighlightMarker;
use crate::map::highlights::range_highlights::{RangeHighlightMarker, RangeHighlights};
use crate::map::highlights::HighlightedTiles;
use crate::map::tile_cursor::position_for_tile;
use crate::map::{MapState, MouseCursorOnTile, TileChangeEvent};
use bevy::app::{App, Update};
use bevy::core::Name;
use bevy::pbr::{NotShadowCaster, PbrBundle};
use bevy::prelude::{
    default, on_event, resource_changed_or_removed, Commands, Component, Condition, Entity,
    IntoSystemConfigs, OnExit, Plugin, Query, Res, Resource, Transform, With,
};
use game_common::game_map::GameMap;

const EXTRA_HEIGHT: f32 = 0.005;

pub struct HighlightPlugin;
impl Plugin for HighlightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                on_highlight_change::<RangeHighlightMarker, RangeHighlights>
                    .run_if(resource_changed_or_removed::<RangeHighlights>()),
                on_highlight_change::<AttackHighlightMarker, AttackHighlights>
                    .run_if(resource_changed_or_removed::<AttackHighlights>()),
                on_highlight_change::<CursorHighlightMarker, MouseCursorOnTile>.run_if(
                    resource_changed_or_removed::<MouseCursorOnTile>()
                        .or_else(on_event::<TileChangeEvent>()),
                ),
            ),
        );
        app.add_systems(OnExit(MapState::Loaded), clean_up);
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
            Name::new(format!(
                "{} Highlight [{},{}]",
                TResource::name(),
                hex.x,
                hex.y
            )),
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

fn clean_up(
    mut commands: Commands,
    cursor: Query<Entity, With<CursorHighlightMarker>>,
    range: Query<Entity, With<RangeHighlightMarker>>,
    attack: Query<Entity, With<AttackHighlightMarker>>,
) {
    for x in cursor.iter() {
        commands.entity(x).despawn();
    }
    for x in range.iter() {
        commands.entity(x).despawn();
    }
    for x in attack.iter() {
        commands.entity(x).despawn();
    }
}
