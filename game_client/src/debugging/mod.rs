mod world_inspector;

use crate::debugging::world_inspector::{WorldInspectorPlugin, WorldInspectorState};
use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

pub struct DebuggingPlugin;
impl Plugin for DebuggingPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<DebugAction>::default())
            .add_plugins(WorldInspectorPlugin)
            .init_resource::<ActionState<DebugAction>>()
            .insert_resource(DebugAction::default_input_map())
            .add_systems(Update, track_input);
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum DebugAction {
    ToggleWorldInspector,
}

impl DebugAction {
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();

        input_map.insert(Self::ToggleWorldInspector, KeyCode::F2);

        input_map
    }
}

#[derive(SystemParam)]
struct CurrentStates<'w> {
    world_inspector: Res<'w, State<WorldInspectorState>>,
}

#[derive(SystemParam)]
struct StateChanges<'w> {
    world_inspector: ResMut<'w, NextState<WorldInspectorState>>,
}

fn track_input(
    input_state: Res<ActionState<DebugAction>>,
    current_states: CurrentStates,
    mut state_changes: StateChanges,
) {
    if input_state.just_pressed(&DebugAction::ToggleWorldInspector) {
        match current_states.world_inspector.get() {
            WorldInspectorState::Off => state_changes.world_inspector.set(WorldInspectorState::On),
            WorldInspectorState::On => state_changes.world_inspector.set(WorldInspectorState::Off),
        }
    }
}
