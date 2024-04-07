mod camera;
mod debugging;
mod load;
mod main_menu;
mod map;
mod map_editor;
mod networking;

use crate::camera::CameraPlugin;
use crate::debugging::DebuggingPlugin;
use crate::load::LoadPlugin;
use crate::main_menu::MainMenuPlugin;
use crate::map::GameMapPlugin;
use crate::map_editor::MapEditorPlugin;
use crate::networking::NetworkPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_kira_audio::AudioPlugin;
use bevy_screen_diagnostics::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(ImagePlugin::default_nearest())
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Your ad could be placed here!".to_string(),
                        present_mode: PresentMode::AutoNoVsync,
                        ..default()
                    }),
                    ..default()
                }),
            AudioPlugin,
        ))
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        .add_plugins(ScreenEntityDiagnosticsPlugin)
        .add_plugins(DebuggingPlugin)
        .add_plugins(MapEditorPlugin)
        .add_plugins(NetworkPlugin)
        .add_plugins(GameMapPlugin)
        .add_plugins(CameraPlugin)
        .add_plugins(LoadPlugin)
        .add_plugins(MainMenuPlugin)
        .init_state::<ApplicationState>()
        .init_state::<MouseCursorOverUiState>()
        .run();
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum MouseCursorOverUiState {
    #[default]
    NotOverUI,
    OverUI,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Reflect)]
pub enum ApplicationState {
    #[default]
    MainMenu,
    MapEditor,
    InGame,
}
