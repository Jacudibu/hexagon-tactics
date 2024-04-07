use crate::map::SpawnMapCommand;
use crate::networking::{Network, NetworkState};
use crate::ApplicationState;
use bevy::app::{App, AppExit, Plugin};
use bevy::prelude::{
    error, in_state, on_event, Commands, EventReader, EventWriter, IntoSystemConfigs, NextState,
    OnEnter, Reflect, ResMut, States, Update,
};
use bevy_egui::egui::Align2;
use bevy_egui::{egui, EguiContexts};
use game_common::game_map::GameMap;
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::server_to_client;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>();
        app.add_systems(OnEnter(ApplicationState::MainMenu), setup);
        app.add_systems(
            Update,
            (
                main_menu.run_if(in_state(MenuState::MainMenu)),
                (
                    connection_menu.run_if(in_state(NetworkState::Disconnected)),
                    (
                        play_menu,
                        load_map_listener
                            .run_if(on_event::<server_to_client::StartGameAndLoadMap>()),
                    )
                        .run_if(in_state(NetworkState::Connected)),
                )
                    .run_if(in_state(MenuState::PlayMenu)),
            )
                .run_if(in_state(ApplicationState::MainMenu)),
        );
    }
}

fn setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::MainMenu);
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Reflect)]
pub enum MenuState {
    #[default]
    MainMenu,
    PlayMenu,
}

fn main_menu(
    mut egui: EguiContexts,
    mut next_application_state: ResMut<NextState<ApplicationState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut app_exit_events: EventWriter<AppExit>,
) {
    egui::Window::new("Main Menu (Super lazy edition)")
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .anchor(Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(egui.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                if ui.button("Map Editor").clicked() {
                    next_application_state.set(ApplicationState::MapEditor);
                }
                if ui.button("Play").clicked() {
                    next_menu_state.set(MenuState::PlayMenu);
                }
                if ui.button("Quit").clicked() {
                    app_exit_events.send(AppExit);
                }
            })
        });
}

fn connection_menu(
    mut egui: EguiContexts,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut network: ResMut<Network>,
) {
    egui::Window::new("Connection Menu")
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .anchor(Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(egui.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                if ui.button("Connect").clicked() {
                    // TODO: Acquire the IP from somewhere
                    network.connect();
                }
                if ui.button("Back to Menu").clicked() {
                    next_menu_state.set(MenuState::MainMenu);
                }
            })
        });
}

fn play_menu(
    mut egui: EguiContexts,
    mut event_writer: EventWriter<ClientToServerMessage>,
    mut network: ResMut<Network>,
) {
    egui::Window::new("Play Menu")
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .anchor(Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(egui.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                if ui.button("Start Game").clicked() {
                    event_writer.send(ClientToServerMessage::StartGame);
                }
                if ui.button("Disconnect").clicked() {
                    network.disconnect();
                }
            })
        });
}

fn load_map_listener(
    mut commands: Commands,
    mut incoming_events: EventReader<server_to_client::StartGameAndLoadMap>,
    mut outgoing_events: EventWriter<SpawnMapCommand>,
    mut next_application_state: ResMut<NextState<ApplicationState>>,
) {
    for event in incoming_events.read() {
        match GameMap::load_from_file(&event.path) {
            Ok(map) => {
                commands.insert_resource(map);
                outgoing_events.send(SpawnMapCommand {});
                next_application_state.set(ApplicationState::InGame);
            }
            Err(e) => {
                error!("Failed to load map {} - error: {:?}", event.path, e)
            }
        }
    }
}
