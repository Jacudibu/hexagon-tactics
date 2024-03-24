use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy_egui::{egui, EguiContext};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum WorldInspectorState {
    #[default]
    Off,
    On,
}

pub struct WorldInspectorPlugin;
impl Plugin for WorldInspectorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bevy_inspector_egui::DefaultInspectorConfigPlugin)
            .add_systems(
                Update,
                world_inspector_ui.run_if(in_state(WorldInspectorState::On)),
            )
            .insert_state(WorldInspectorState::Off);
    }
}

fn world_inspector_ui(world: &mut World) {
    let Ok(egui_context) = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world)
    else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new("World Inspector").show(egui_context.get_mut(), |ui| {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // equivalent to `WorldInspectorPlugin`
            bevy_inspector_egui::bevy_inspector::ui_for_world(world, ui);

            egui::CollapsingHeader::new("Materials").show(ui, |ui| {
                bevy_inspector_egui::bevy_inspector::ui_for_assets::<StandardMaterial>(world, ui);
            });

            // ui.heading("Entities");
            // bevy_inspector_egui::bevy_inspector::ui_for_world_entities(world, ui);
        });
    });
}
