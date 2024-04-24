use crate::map::map_plugin::MapState;
use crate::map::{MouseCursorOnTile, METERS_PER_TILE_HEIGHT_UNIT};
use bevy::app::{App, Plugin, Startup, Update};
use bevy::math::Vec3;
use bevy::prelude::{
    error, in_state, AppGizmoBuilder, Color, GizmoConfigGroup, GizmoConfigStore, Gizmos,
    IntoSystemConfigs, Reflect, Res, ResMut,
};
use game_common::game_map::{GameMap, HEX_LAYOUT};
use hexx::{GridVertex, HexLayout};

pub(in crate::map) struct MapGizmosPlugin;
impl Plugin for MapGizmosPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<HexagonOutlineGizmos>();
        app.init_gizmo_group::<CursorGizmos>();
        app.add_systems(Startup, setup_gizmo_config);
        app.add_systems(
            Update,
            (draw_hexagon_gizmos, draw_cursor_gizmos).run_if(in_state(MapState::Loaded)),
        );
    }
}

#[derive(Default, Reflect, GizmoConfigGroup)]
struct HexagonOutlineGizmos;
#[derive(Default, Reflect, GizmoConfigGroup)]
struct CursorGizmos;

fn draw_hexagon_gizmos(mut gizmos: Gizmos<HexagonOutlineGizmos>, map: Res<GameMap>) {
    for (hex, data) in &map.tiles {
        let height = data.height as f32 * METERS_PER_TILE_HEIGHT_UNIT;
        if data.height == 0 {
            continue;
        }

        let top_vertices = hex
            .all_vertices()
            .map(|x| vertex_coordinates_3d(&HEX_LAYOUT, x, height));

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
            .map(|x| vertex_coordinates_3d(&HEX_LAYOUT, x, 0.0));

        gizmos.line(top_vertices[0], bottom_vertices[0], Color::BLACK);
        gizmos.line(top_vertices[1], bottom_vertices[1], Color::BLACK);
        gizmos.line(top_vertices[2], bottom_vertices[2], Color::BLACK);
        gizmos.line(top_vertices[3], bottom_vertices[3], Color::BLACK);
        gizmos.line(top_vertices[4], bottom_vertices[4], Color::BLACK);
        gizmos.line(top_vertices[5], bottom_vertices[5], Color::BLACK);
    }
}

fn draw_cursor_gizmos(
    cursor: Option<Res<MouseCursorOnTile>>,
    mut gizmos: Gizmos<CursorGizmos>,
    map: Res<GameMap>,
) {
    let Some(cursor) = cursor else {
        return;
    };

    let Some(tile) = map.tiles.get(&cursor.hex) else {
        error!("Was unable to get map tile at {:?}", cursor.hex);
        return;
    };

    let Some(fluid) = &tile.fluid else {
        return;
    };

    let top_vertices = cursor.hex.all_vertices().map(|x| {
        vertex_coordinates_3d(
            &HEX_LAYOUT,
            x,
            (tile.height as f32 + fluid.height) * METERS_PER_TILE_HEIGHT_UNIT,
        )
    });

    let bottom_vertices = cursor.hex.all_vertices().map(|x| {
        vertex_coordinates_3d(
            &HEX_LAYOUT,
            x,
            tile.height as f32 * METERS_PER_TILE_HEIGHT_UNIT,
        )
    });

    gizmos.line(top_vertices[0], bottom_vertices[0], Color::BLACK);
    gizmos.line(top_vertices[1], bottom_vertices[1], Color::BLACK);
    gizmos.line(top_vertices[2], bottom_vertices[2], Color::BLACK);
    gizmos.line(top_vertices[3], bottom_vertices[3], Color::BLACK);
    gizmos.line(top_vertices[4], bottom_vertices[4], Color::BLACK);
    gizmos.line(top_vertices[5], bottom_vertices[5], Color::BLACK);
}

fn connect_hexagon_vertices<T: GizmoConfigGroup>(gizmos: &mut Gizmos<T>, vertices: [Vec3; 6]) {
    gizmos.line(vertices[0], vertices[1], Color::BLACK);
    gizmos.line(vertices[1], vertices[2], Color::BLACK);
    gizmos.line(vertices[2], vertices[3], Color::BLACK);
    gizmos.line(vertices[3], vertices[4], Color::BLACK);
    gizmos.line(vertices[4], vertices[5], Color::BLACK);
    gizmos.line(vertices[5], vertices[0], Color::BLACK);
}

#[must_use]
fn vertex_coordinates_3d(layout: &HexLayout, vertex: GridVertex, height: f32) -> Vec3 {
    let vertex_coordinates = layout.vertex_coordinates(vertex);
    Vec3 {
        x: vertex_coordinates.x,
        y: height,
        z: vertex_coordinates.y,
    }
}

fn setup_gizmo_config(mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<HexagonOutlineGizmos>();
    config.depth_bias = -0.00001;
    config.line_width = 20.0;
    config.line_perspective = true;

    let (config, _) = config_store.config_mut::<CursorGizmos>();
    config.depth_bias = -0.00001;
    config.line_width = 20.0;
    config.line_perspective = true;
}
