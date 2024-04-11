use crate::map_editor::*;
use crate::ApplicationState;
use bevy::app::{App, Plugin};
use bevy_egui::egui::{Align2, Pos2};
use bevy_egui::{egui, EguiContexts, EguiPlugin};
use game_common::game_map::GameMap;
use game_common::TEST_MAP_NAME;

pub struct MapEditorUiPlugin;
impl Plugin for MapEditorUiPlugin {
    fn build(&self, app: &mut App) {
        if !app.is_plugin_added::<EguiPlugin>() {
            app.add_plugins(EguiPlugin);
        }

        app.add_systems(
            Update,
            (tool_view, menu_buttons).run_if(in_state(ApplicationState::MapEditor)),
        );
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

fn menu_buttons(
    mut commands: Commands,
    mut egui: EguiContexts,
    map: Res<GameMap>,
    mut next_application_state: ResMut<NextState<ApplicationState>>,
    mut spawn_new_world_command: EventWriter<SpawnMapCommand>,
) {
    egui::Window::new("Save & Load")
        .title_bar(false)
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::RIGHT_TOP, egui::Vec2::ZERO)
        .fixed_pos(Pos2::new(-5.0, -5.0))
        .show(egui.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                if ui.button("New").clicked() {
                    commands.insert_resource(GameMap::new(10));
                    spawn_new_world_command.send(SpawnMapCommand {});
                }
                if ui.button("Save").clicked() {
                    map.write_to_disk(TEST_MAP_NAME);
                }
                if ui.button("Load").clicked() {
                    match GameMap::load_from_file(TEST_MAP_NAME) {
                        Ok(map) => {
                            commands.insert_resource(map);
                            spawn_new_world_command.send(SpawnMapCommand {});
                        }
                        Err(_) => {
                            // TODO: Error response. Low priority.
                        }
                    }
                }
                if ui.button("Back To Menu").clicked() {
                    next_application_state.set(ApplicationState::MainMenu);
                }
            })
        });
}
