mod debugging;

use bevy::prelude::*;
use bevy::window::PresentMode;
use bevy_basic_camera::CameraControllerPlugin;
use bevy_kira_audio::AudioPlugin;
use bevy_screen_diagnostics::*;
use crate::debugging::DebuggingPlugin;

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
    .run();
}
