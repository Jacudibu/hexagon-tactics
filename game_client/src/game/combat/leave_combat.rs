use crate::game::combat::combat_plugin::CombatState;
use crate::game::combat::local_combat_data::LocalCombatData;
use crate::game::game_plugin::GameState;
use crate::map::DespawnMapCommand;
use crate::ApplicationState;
use bevy::app::App;
use bevy::prelude::*;
use game_common::combat_data::CombatData;

pub struct LeaveCombatPlugin;
impl Plugin for LeaveCombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LeaveCombatCommand>();
        app.add_systems(
            Update,
            on_leave_combat.run_if(on_event::<LeaveCombatCommand>()),
        );
    }
}

#[derive(Event)]
pub struct LeaveCombatCommand; // TODO: Should probably contain information on where to go next

pub fn on_leave_combat(
    mut commands: Commands,
    mut event_reader: EventReader<LeaveCombatCommand>,
    mut despawn_map_command: EventWriter<DespawnMapCommand>,
    mut next_combat_state: ResMut<NextState<CombatState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_application_state: ResMut<NextState<ApplicationState>>,
) {
    for _event in event_reader.read() {
        commands.remove_resource::<CombatData>();
        commands.remove_resource::<LocalCombatData>();
        despawn_map_command.send(DespawnMapCommand {});
        next_combat_state.set(CombatState::WaitingForServer);
        next_game_state.set(GameState::Inactive);
        next_application_state.set(ApplicationState::MainMenu);
    }
}
