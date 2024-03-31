mod debugging;
mod game_map;
mod map_editor;
mod networking;

use crate::debugging::DebuggingPlugin;
use crate::game_map::GameMapPlugin;
use crate::map_editor::MapEditorPlugin;
use crate::networking::{Network, NetworkPlugin};
use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_basic_camera::{CameraController, CameraControllerPlugin};
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
        .add_plugins(CameraControllerPlugin)
        .add_plugins(ScreenDiagnosticsPlugin::default())
        .add_plugins(ScreenFrameDiagnosticsPlugin)
        .add_plugins(ScreenEntityDiagnosticsPlugin)
        .add_plugins(DebuggingPlugin)
        .add_plugins(NetworkPlugin)
        .add_systems(Startup, init)
        .add_plugins(GameMapPlugin)
        .run();
}

// Placeholder
fn init(mut network: ResMut<Network>) {
    network.connect();
}
