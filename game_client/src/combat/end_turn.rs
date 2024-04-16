use crate::combat::combat_plugin::CombatState;
use bevy::app::App;
use bevy::prelude::{
    on_event, Event, EventWriter, IntoSystemConfigs, NextState, Plugin, ResMut, Update,
};
use game_common::network_events::client_to_server::ClientToServerMessage;

pub struct EndTurnPlugin;
impl Plugin for EndTurnPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<EndTurnCommand>();
        app.add_systems(Update, end_turn.run_if(on_event::<EndTurnCommand>()));
    }
}

#[derive(Event)]
pub struct EndTurnCommand;

pub fn end_turn(
    mut network: EventWriter<ClientToServerMessage>,
    mut next_combat_state: ResMut<NextState<CombatState>>,
) {
    network.send(ClientToServerMessage::EndTurn);
    next_combat_state.set(CombatState::WaitingForServer);
}