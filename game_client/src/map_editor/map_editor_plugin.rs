use bevy::app::{App, Plugin, Update};
use bevy::prelude::{
    in_state, Commands, EventWriter, IntoSystemConfigs, OnEnter, OnExit, PreUpdate, Res, ResMut,
};
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::plugin::InputManagerPlugin;

use game_common::game_map::GameMap;

use crate::map::{DespawnMapCommand, MapState, SpawnMapCommand};
use crate::map_editor::editor_ui::MapEditorUiPlugin;
use crate::map_editor::map_editor_action::{MapEditorAction, ACTION_TO_TOOL};
use crate::map_editor::map_editor_tool::{use_tool, MapEditorTool};
use crate::map_editor::tools::MapEditorToolsPlugin;
use crate::{ApplicationState, MouseCursorOverUiState};

pub struct MapEditorPlugin;

impl Plugin for MapEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<MapEditorAction>::default());
        app.add_plugins(MapEditorUiPlugin);
        app.add_plugins(MapEditorToolsPlugin);
        app.init_resource::<ActionState<MapEditorAction>>();
        app.insert_resource(MapEditorAction::default_input_map());
        app.add_systems(OnEnter(ApplicationState::MapEditor), setup_map_editor);
        app.add_systems(OnExit(ApplicationState::MapEditor), exit_map_editor);
        app.add_systems(
            PreUpdate,
            switch_tool_on_button_press
                .run_if(in_state(ApplicationState::MapEditor))
                .run_if(in_state(MapState::Loaded)),
        );
        app.add_systems(
            Update,
            use_tool
                .run_if(in_state(MouseCursorOverUiState::NotOverUI))
                .run_if(in_state(ApplicationState::MapEditor))
                .run_if(in_state(MapState::Loaded)),
        );
    }
}

fn setup_map_editor(mut commands: Commands, mut spawn_map_command: EventWriter<SpawnMapCommand>) {
    commands.insert_resource(MapEditorTool::default());
    let default_radius = 10;
    let map = GameMap::new(default_radius);
    commands.insert_resource(map);

    spawn_map_command.send(SpawnMapCommand {});
}

fn exit_map_editor(mut despawn_map_command: EventWriter<DespawnMapCommand>) {
    despawn_map_command.send(DespawnMapCommand {});
}

fn switch_tool_on_button_press(
    input_state: Res<ActionState<MapEditorAction>>,
    mut current_tool: ResMut<MapEditorTool>,
) {
    for (action, tool) in ACTION_TO_TOOL {
        if input_state.just_pressed(&action) {
            *current_tool = tool;
            break;
        }
    }
}
