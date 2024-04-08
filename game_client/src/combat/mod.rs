use crate::map::MapState;
use crate::ApplicationState;
use bevy::app::App;
use bevy::prelude::{in_state, EventWriter, IntoSystemConfigs, OnEnter, Plugin, Reflect, States};
use game_common::network_events::client_to_server::ClientToServerMessage;

pub struct CombatPlugin;
impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(MapState::Loaded),
            on_finish_loading.run_if(in_state(ApplicationState::InGame)),
        );
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Reflect)]
pub enum CombatState {
    #[default]
    WaitingForServer,
    LocalPlayerTurn,
}

pub fn on_finish_loading(mut network_event: EventWriter<ClientToServerMessage>) {
    network_event.send(ClientToServerMessage::FinishedLoading);
}
