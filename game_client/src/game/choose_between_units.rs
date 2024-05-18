use crate::game::game_plugin::GameState;
use crate::game::ui_utils;
use crate::networking::NetworkState;
use bevy::app::{App, Plugin};
use bevy::prelude::{
    in_state, on_event, Commands, Event, EventReader, EventWriter, IntoSystemConfigs, NextState,
    Res, ResMut, Resource, Update,
};
use bevy_egui::egui::{Align2, Pos2, RichText, TextStyle};
use bevy_egui::{egui, EguiContexts};
use game_common::game_data::unit_definition::UnitDefinition;
use game_common::game_data::GameData;
use game_common::network_events::client_to_server::{ClientToServerMessage, PickUnit};
use game_common::network_events::server_to_client;

pub struct ChooseBetweenUnitsPlugin;
impl Plugin for ChooseBetweenUnitsPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ChooseUnitButtonPress>();
        app.add_systems(
            Update,
            choose_between_units_listener
                .run_if(on_event::<server_to_client::ChooseBetweenUnits>())
                .run_if(in_state(NetworkState::Connected)),
        );

        app.add_systems(
            Update,
            (choose_between_units_ui, on_choose_unit)
                .run_if(in_state(GameState::ChooseBetweenUnits)),
        );
    }
}

pub fn choose_between_units_listener(
    mut commands: Commands,
    mut incoming_events: EventReader<server_to_client::ChooseBetweenUnits>,
    mut next_application_state: ResMut<NextState<GameState>>,
) {
    for event in incoming_events.read() {
        commands.insert_resource(ChooseBetweenUnitsResource {
            units: event.units.clone(),
        });
        next_application_state.set(GameState::ChooseBetweenUnits);
    }
}

#[derive(Event)]
struct ChooseUnitButtonPress {
    pub unit_id: u32,
}

fn on_choose_unit(
    mut button_press: EventReader<ChooseUnitButtonPress>,
    mut client_to_server_messages: EventWriter<ClientToServerMessage>,
) {
    for event in button_press.read() {
        client_to_server_messages.send(ClientToServerMessage::PickUnit(PickUnit {
            unit_id: event.unit_id,
        }));
    }
}

fn choose_between_units_ui(
    mut egui: EguiContexts,
    units: Res<ChooseBetweenUnitsResource>,
    game_data: Res<GameData>,
    mut on_button_press: EventWriter<ChooseUnitButtonPress>,
) {
    egui::Window::new("Pick a Unit")
        .title_bar(true)
        .collapsible(false)
        .resizable(false)
        .anchor(Align2::CENTER_CENTER, egui::Vec2::new(0.0, 0.0))
        .fixed_pos(Pos2::new(0.0, 0.0))
        .show(egui.ctx_mut(), |ui| {
            ui.set_max_width(1920.0);
            ui.horizontal(|ui| {
                for unit in &units.units {
                    ui.vertical(|ui| {
                        ui.label(
                            RichText::new(format!("{}", unit.name)).text_style(TextStyle::Heading),
                        );
                        ui_utils::print_unit_definition_info(ui, unit, &game_data);
                        if ui.button("Pick This").clicked() {
                            on_button_press.send(ChooseUnitButtonPress { unit_id: unit.id });
                        }
                    });
                }
            });
        });
}

#[derive(Resource)]
struct ChooseBetweenUnitsResource {
    pub units: Vec<UnitDefinition>,
}
