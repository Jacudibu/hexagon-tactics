use crate::load::{HexagonMeshes, HighlightMaterials};
use crate::map::highlights::active_unit_highlights::{
    ActiveUnitHighlightMarker, ActiveUnitHighlights,
};
use crate::map::highlights::attack_highlights::{AttackHighlightMarker, AttackHighlights};
use crate::map::highlights::cursor_highlights::CursorHighlightMarker;
use crate::map::highlights::path_highlights::PathHighlightMarker;
use crate::map::highlights::range_highlights::{RangeHighlightMarker, RangeHighlights};
use crate::map::highlights::HighlightedTiles;
use crate::map::{
    CursorOnTile, MapState, PathHighlights, TileChangeEvent, METERS_PER_TILE_HEIGHT_UNIT,
};
use bevy::app::{App, Update};
use bevy::core::Name;
use bevy::log::error;
use bevy::math::Vec3;
use bevy::pbr::{NotShadowCaster, PbrBundle};
use bevy::prelude::{
    default, in_state, on_event, resource_changed_or_removed, Commands, Component, Condition,
    Entity, IntoSystemConfigs, OnExit, Plugin, Query, Res, Resource, Transform, With,
};
use game_common::game_map::{GameMap, HEX_LAYOUT};
use hexx::Hex;

pub struct HighlightPlugin;
impl Plugin for HighlightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                on_highlight_change::<RangeHighlightMarker, RangeHighlights>
                    .run_if(resource_changed_or_removed::<RangeHighlights>()),
                on_highlight_change::<PathHighlightMarker, PathHighlights>
                    .run_if(resource_changed_or_removed::<PathHighlights>()),
                on_highlight_change::<AttackHighlightMarker, AttackHighlights>
                    .run_if(resource_changed_or_removed::<AttackHighlights>()),
                on_highlight_change::<ActiveUnitHighlightMarker, ActiveUnitHighlights>
                    .run_if(resource_changed_or_removed::<ActiveUnitHighlights>()),
                on_highlight_change::<CursorHighlightMarker, CursorOnTile>.run_if(
                    resource_changed_or_removed::<CursorOnTile>()
                        .or_else(on_event::<TileChangeEvent>()),
                ),
            ),
        );
        app.add_systems(OnExit(MapState::Ready), clean_up);
    }
}

fn on_highlight_change<TMarker: Component + Default, TResource: Resource + HighlightedTiles>(
    mut commands: Commands,
    existing_highlights: Query<Entity, With<TMarker>>,
    highlighted_tiles: Option<Res<TResource>>,
    map: Option<Res<GameMap>>,
    hexagon_meshes: Res<HexagonMeshes>,
    cursor_materials: Res<HighlightMaterials>,
) {
    for entity in existing_highlights.iter() {
        commands.entity(entity).despawn();
    }

    let Some(highlighted_tiles) = highlighted_tiles else {
        return;
    };

    let Some(map) = map else {
        return;
    };

    for hex in highlighted_tiles.tiles() {
        let translation = highlight_position(&map, hex, TResource::extra_height());

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

fn clean_up(mut commands: Commands) {
    commands.remove_resource::<CursorOnTile>();
    commands.remove_resource::<RangeHighlights>();
    commands.remove_resource::<AttackHighlights>();
    commands.remove_resource::<ActiveUnitHighlights>();
    commands.remove_resource::<PathHighlights>();
}

fn highlight_position(map: &GameMap, hex: &Hex, extra_height: f32) -> Vec3 {
    let position = HEX_LAYOUT.hex_to_world_pos(hex.clone());
    let height = if let Some(tile) = map.tiles.get(hex) {
        if let Some(fluid) = &tile.fluid {
            tile.height as f32 + fluid.height
        } else {
            tile.height as f32
        }
    } else {
        error!("Was unable to find a tile for {:?} in map.", hex);
        0.0
    };

    Vec3 {
        x: position.x,
        y: height * METERS_PER_TILE_HEIGHT_UNIT + extra_height,
        z: position.y,
    }
}
