use crate::networking::SendNetworkMessage;
use crate::GameState;
use bevy::app::AppExit;
use bevy::prelude::*;
use bevy_egui::egui::Align2;
use bevy_egui::{egui, EguiContexts};
use game_common::network_message::NetworkMessage;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Menu), setup);
        app.add_systems(Update, draw_menu.run_if(in_state(GameState::Menu)));
    }
}

fn setup() {
    // Currently there's just nothing to do, but later on we need to actually spawn the system here
}

fn draw_menu(
    mut egui: EguiContexts,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut app_exit_events: EventWriter<AppExit>,
    mut event_writer: EventWriter<SendNetworkMessage>,
) {
    egui::Window::new("Main Menu (Super lazy edition)")
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .anchor(Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(egui.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                if ui.button("Map Editor").clicked() {
                    next_game_state.set(GameState::MapEditor);
                }
                if ui.button("Play").clicked() {
                    next_game_state.set(GameState::Game);
                    event_writer.send(SendNetworkMessage::new(NetworkMessage::StartGame));
                }
                if ui.button("Quit").clicked() {
                    app_exit_events.send(AppExit);
                }
            })
        });
}
