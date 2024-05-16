use crate::game::combat::combat_plugin::CombatState;
use crate::game::combat::local_combat_data::LocalCombatData;
use crate::game::combat::unit_placement::UnitMarker;
use crate::game::game_plugin::GameState;
use crate::map::DespawnMapCommand;
use crate::ApplicationState;
use bevy::app::App;
use bevy::prelude::*;
use game_common::combat_data::CombatData;
use game_common::network_events::client_to_server::ClientToServerMessage;

pub struct LeaveCombatPlugin;
impl Plugin for LeaveCombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<LeaveCombatCommand>();
        app.add_systems(
            Update,
            on_leave_combat.run_if(on_event::<LeaveCombatCommand>()),
        );
        app.add_systems(
            PostUpdate,
            unload_units.run_if(on_event::<DespawnMapCommand>()),
        );
    }
}

#[derive(Event)]
pub struct LeaveCombatCommand {
    how: LeaveCombatAction,
}

impl LeaveCombatCommand {
    pub fn proceed() -> Self {
        LeaveCombatCommand {
            how: LeaveCombatAction::ProceedAfterVictory,
        }
    }

    pub fn quit() -> Self {
        LeaveCombatCommand {
            how: LeaveCombatAction::BackToMainMenu,
        }
    }
}

enum LeaveCombatAction {
    BackToMainMenu,
    ProceedAfterVictory,
}

pub fn on_leave_combat(
    mut commands: Commands,
    mut event_reader: EventReader<LeaveCombatCommand>,
    mut despawn_map_command: EventWriter<DespawnMapCommand>,
    mut next_combat_state: ResMut<NextState<CombatState>>,
    mut next_game_state: ResMut<NextState<GameState>>,
    mut next_application_state: ResMut<NextState<ApplicationState>>,
    mut client_to_server_messages: EventWriter<ClientToServerMessage>,
) {
    for event in event_reader.read() {
        commands.remove_resource::<CombatData>();
        commands.remove_resource::<LocalCombatData>();
        despawn_map_command.send(DespawnMapCommand {});
        next_combat_state.set(CombatState::WaitingForServer);
        next_game_state.set(GameState::Inactive);

        match event.how {
            LeaveCombatAction::BackToMainMenu => {
                next_application_state.set(ApplicationState::MainMenu);
            }
            LeaveCombatAction::ProceedAfterVictory => {
                client_to_server_messages.send(ClientToServerMessage::Proceed);
            }
        }
    }
}

pub fn unload_units(mut commands: Commands, units: Query<Entity, With<UnitMarker>>) {
    for x in units.iter() {
        commands.entity(x).despawn_recursive();
    }
}
