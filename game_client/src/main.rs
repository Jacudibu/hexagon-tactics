mod camera;
mod combat;
mod combat_data_resource;
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
use crate::networking::NetworkPlugin;
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_kira_audio::AudioPlugin;
use bevy_screen_diagnostics::*;
use bevy_sprite3d::Sprite3dPlugin;
use combat::CombatPlugin;
use map::GameMapPlugin;
use map_editor::MapEditorPlugin;

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
                })
                .set(bevy::log::LogPlugin {
                    level: bevy::log::Level::DEBUG,
                    filter: "bevy=info,wgpu=error,naga=warn,mygame=debug".into(),
                    ..default()
                }),
            AudioPlugin,
        ))
        .add_plugins(Sprite3dPlugin)
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
        .add_plugins(CombatPlugin)
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
