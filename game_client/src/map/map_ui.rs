use crate::map::tile_cursor::MouseCursorOnTile;
use crate::MouseCursorOverUiState;
use bevy::app::{App, First, Plugin, Update};
use bevy::prelude::*;
use bevy_egui::egui::Pos2;
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use game_common::game_map::GameMap;

pub(in crate::map) struct MapUiPlugin;
impl Plugin for MapUiPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }

        app.init_state::<MouseCursorOverUiState>()
            .add_systems(First, detect_mouse_cursor_over_ui)
            .add_systems(
                Update,
                tile_cursor_ui.run_if(resource_exists::<MouseCursorOnTile>),
            );
    }
}

fn detect_mouse_cursor_over_ui(
    mut egui: EguiContexts,
    current_mouse_state: Res<State<MouseCursorOverUiState>>,
    mut next_state: ResMut<NextState<MouseCursorOverUiState>>,
) {
    if egui.ctx_mut().is_pointer_over_area() {
        if current_mouse_state.get() != &MouseCursorOverUiState::OverUI {
            next_state.set(MouseCursorOverUiState::OverUI);
        }
    } else {
        if current_mouse_state.get() != &MouseCursorOverUiState::NotOverUI {
            next_state.set(MouseCursorOverUiState::NotOverUI);
        }
    }
}

fn tile_cursor_ui(mut egui: EguiContexts, cursor: Res<MouseCursorOnTile>, map: Res<GameMap>) {
    let text = if let Some(tile) = map.tiles.get(&cursor.hex) {
        if tile.height == 0 {
            return;
        }

        let mut lines = Vec::new();
        lines.push(format!("Height: {}", tile.height));
        lines.push(format!("Surface: {}", tile.surface));
        lines.join("\n")
    } else {
        String::from("Unable to find tile in Map?!")
    };

    egui::Window::new(format!("[{},{}]", cursor.hex.x, cursor.hex.y))
        .collapsible(false)
        .resizable(false)
        .fixed_pos(Pos2::new(5.0, 5.0))
        .show(egui.ctx_mut(), |ui| ui.label(text));
}
