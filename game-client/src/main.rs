mod debugging;
mod networking;

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_basic_camera::{CameraController, CameraControllerPlugin};
use bevy_kira_audio::AudioPlugin;
use bevy_screen_diagnostics::*;
use crate::debugging::DebuggingPlugin;
use crate::networking::{Network, NetworkPlugin};

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
    .run();
}

// Placeholder
fn init(mut commands: Commands, mut network: ResMut<Network>) {
    commands.spawn(Camera3dBundle::default()).insert(CameraController::default());
    network.connect();
}
