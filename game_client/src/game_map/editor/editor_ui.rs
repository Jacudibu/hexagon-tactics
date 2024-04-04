use crate::game_map::editor::{MapEditorAction, MapEditorTool, ACTION_TO_TOOL};
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use bevy_egui::egui::{Align2, Pos2};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use game_common::game_map::GameMap;

pub(in crate::game_map::editor) struct EditorUiPlugin;
impl Plugin for EditorUiPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }

        app.add_systems(Update, (tool_view, io_buttons));
    }
}

fn tool_view(mut egui: EguiContexts, mut current_tool: ResMut<MapEditorTool>) {
    egui::Window::new("Editor Buttons")
        .title_bar(false)
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::LEFT_BOTTOM, egui::Vec2::new(0.0, 0.0))
        .fixed_pos(Pos2::new(5.0, 5.0))
        .show(egui.ctx_mut(), |ui| {
            for (action, tool) in ACTION_TO_TOOL {
                if action == MapEditorAction::UseTool {
                    continue;
                }

                if ui.button(action.to_string()).clicked() {
                    *current_tool = tool;
                }
            }

            ui.label(format!("Current: {}", *current_tool));
        });
}

fn io_buttons(mut egui: EguiContexts, map: Res<GameMap>) {
    egui::Window::new("Save & Load")
        .title_bar(false)
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::RIGHT_TOP, egui::Vec2::new(0.0, 0.0))
        .fixed_pos(Pos2::new(-5.0, -5.0))
        .show(egui.ctx_mut(), |ui| {
            if ui.button("Save").clicked() {
                map.write_to_disk("todo.hexmap");
            }
            if ui.button("Load").clicked() {
                let _ = GameMap::load_from_file("todo.hexmap");
            }
        });
}
