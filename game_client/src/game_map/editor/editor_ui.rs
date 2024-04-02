use crate::game_map::editor::MapEditorTool;
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_egui::egui::{Align2, Pos2};
use bevy_egui::{egui, EguiContexts, EguiPlugin};

pub(in crate::game_map::editor) struct EditorUiPlugin;
impl Plugin for EditorUiPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }

        app.add_systems(Update, currently_selected_tool);
    }
}

fn currently_selected_tool(mut egui: EguiContexts, tool: Res<MapEditorTool>) {
    egui::Window::new("Editor Tool View")
        .title_bar(false)
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::LEFT_BOTTOM, egui::Vec2::new(0.0, 0.0))
        .fixed_pos(Pos2::new(5.0, 5.0))
        .show(egui.ctx_mut(), |ui| ui.label(tool.to_string()));
}
