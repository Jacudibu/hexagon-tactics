use crate::map::SpawnMapCommand;
use crate::networking::{Network, NetworkState};
use crate::ApplicationState;
use bevy::app::{App, AppExit, Plugin};
use bevy::prelude::{
    error, in_state, info, on_event, resource_exists, warn, Commands, Event, EventReader,
    EventWriter, IntoSystemConfigs, NextState, OnEnter, Reflect, Res, ResMut, Resource, State,
    States, Update,
};
use bevy_egui::egui::{Align2, Button};
use bevy_egui::{egui, EguiContexts};
use game_common::game_map::GameMap;
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::server_to_client;
use std::process::Child;

pub struct MainMenuPlugin;
impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<MenuState>();
        app.add_event::<HostLocalServerCommand>();
        app.add_systems(OnEnter(ApplicationState::MainMenu), setup);
        app.add_systems(
            Update,
            (
                main_menu.run_if(in_state(MenuState::MainMenu)),
                (
                    (
                        connection_menu,
                        host_local_server.run_if(on_event::<HostLocalServerCommand>()),
                        connect_to_local_host.run_if(resource_exists::<LocalHost>),
                    )
                        .run_if(in_state(NetworkState::Disconnected)),
                    (connection_menu).run_if(in_state(NetworkState::Connecting)),
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
    mut network: ResMut<Network>,
    network_state: Res<State<NetworkState>>,
    mut next_network_state: ResMut<NextState<NetworkState>>,
    mut next_menu_state: ResMut<NextState<MenuState>>,
    mut host_command: EventWriter<HostLocalServerCommand>,
) {
    egui::Window::new("Connection Menu")
        .collapsible(false)
        .resizable(false)
        .movable(false)
        .anchor(Align2::CENTER_CENTER, egui::Vec2::ZERO)
        .show(egui.ctx_mut(), |ui| {
            ui.vertical_centered(|ui| {
                ui.add_enabled_ui(network_state.get() != &NetworkState::Connecting, |ui| {
                    if ui.button("Host").clicked() {
                        host_command.send(HostLocalServerCommand {});
                    }
                    if ui.button("Connect").clicked() {
                        // TODO: Acquire the IP from somewhere
                        network.connect();
                        next_network_state.set(NetworkState::Connecting);
                    }
                });
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

#[derive(Resource)]
struct LocalHost {
    child: Child,
}

fn connect_to_local_host(
    mut network: ResMut<Network>,
    mut connection_state: ResMut<NextState<NetworkState>>,
) {
    network.connect();
    connection_state.set(NetworkState::Connecting);
}

#[derive(Event)]
struct HostLocalServerCommand {}

fn host_local_server(mut commands: Commands, existing_local_server: Option<ResMut<LocalHost>>) {
    if let Some(mut existing_local_server) = existing_local_server {
        warn!("A local host was already running. Attempting to kill it.");
        if let Err(e) = existing_local_server.child.kill() {
            error!(
                "Failed to kill existing local host. Cancelling operation. Error: {:?}",
                e
            );
            return;
        }
    }

    let args = Vec::<String>::new();
    match std::process::Command::new("target/debug/game-server")
        .args(args)
        .spawn()
    {
        Ok(child) => {
            info!("Launched server!");
            commands.insert_resource(LocalHost { child })
        }
        Err(e) => {
            info!("Something went wrong when launching server! {:?}", e)
        }
    }
}
