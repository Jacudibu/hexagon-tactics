use crate::game::choose_between_units;
use crate::game::combat::CombatPlugin;
use crate::map::SpawnMapCommand;
use crate::networking::NetworkState;
use bevy::app::{App, Plugin};
use bevy::log::error;
use bevy::prelude::*;
use game_common::game_map::GameMap;
use game_common::network_events::server_to_client;

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CombatPlugin);
        app.add_plugins(choose_between_units::ChooseBetweenUnitsPlugin);
        app.init_state::<GameState>();
        app.add_systems(
            Update,
            load_map_listener
                .run_if(on_event::<server_to_client::LoadMap>())
                .run_if(in_state(NetworkState::Connected)),
        );
    }
}

fn load_map_listener(
    mut commands: Commands,
    mut incoming_events: EventReader<server_to_client::LoadMap>,
    mut outgoing_events: EventWriter<SpawnMapCommand>,
    mut next_application_state: ResMut<NextState<GameState>>,
) {
    for event in incoming_events.read() {
        match GameMap::load_from_file(&event.path) {
            Ok(map) => {
                commands.insert_resource(map);
                outgoing_events.send(SpawnMapCommand {});
                next_application_state.set(GameState::Combat);
            }
            Err(e) => {
                error!("Failed to load map {} - error: {:?}", event.path, e)
            }
        }
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Reflect)]
pub enum GameState {
    #[default]
    Inactive,
    Combat,
    ChooseBetweenUnits, // Maybe this should be a substate (EventState or so?)
}
