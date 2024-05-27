use crate::game::combat::combat_input::CombatInputPlugin;
use crate::game::combat::combat_ui::CombatUiPlugin;
use crate::game::combat::end_turn::EndTurnPlugin;
use crate::game::combat::leave_combat::LeaveCombatPlugin;
use crate::game::combat::local_combat_data::LocalCombatData;
use crate::game::combat::unit_actions::UnitActionPlugin;
use crate::game::combat::unit_animations::UnitAnimationPlugin;
use crate::game::combat::unit_placement::UnitPlacementPlugin;
use crate::map::{ActiveUnitHighlights, MapState};
use crate::networking::LocalPlayerId;
use crate::ApplicationState;
use bevy::app::{App, Plugin, Update};
use bevy::prelude::{
    error, in_state, on_event, Commands, EventReader, EventWriter, IntoSystemConfigs, NextState,
    OnEnter, Reflect, Res, ResMut, States,
};
use game_common::combat_data::CombatData;
use game_common::combat_turn::CombatTurn;
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::server_to_client::{
    CombatFinished, PlayerTurnToPlaceUnit, StartUnitTurn,
};
use game_common::player_resources::PlayerResources;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CombatUiPlugin);
        app.add_plugins(CombatInputPlugin);
        app.add_plugins(UnitPlacementPlugin);
        app.add_plugins(UnitActionPlugin);
        app.add_plugins(UnitAnimationPlugin);
        app.add_plugins(EndTurnPlugin);
        app.add_plugins(LeaveCombatPlugin);
        app.init_state::<CombatState>();
        app.add_systems(
            OnEnter(MapState::Ready),
            on_map_loaded.run_if(in_state(ApplicationState::InGame)),
        );
        app.add_systems(
            Update,
            (
                on_player_turn_to_place_unit.run_if(on_event::<PlayerTurnToPlaceUnit>()),
                on_start_unit_turn.run_if(on_event::<StartUnitTurn>()),
                on_combat_finished.run_if(on_event::<CombatFinished>()),
            )
                .run_if(in_state(ApplicationState::InGame)),
        );
    }
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States, Reflect)]
pub enum CombatState {
    #[default]
    WaitingForServer,
    WaitingForOtherPlayer,
    PlaceUnit,
    ThisPlayerUnitTurn,
    Victory,
    Defeated,
}

pub fn on_map_loaded(
    mut commands: Commands,
    mut client_to_server_messages: EventWriter<ClientToServerMessage>,
) {
    client_to_server_messages.send(ClientToServerMessage::FinishedLoading);
    commands.insert_resource(CombatData {
        units: Default::default(),
        unit_positions: Default::default(),
        current_turn: CombatTurn::Undefined,
    });
    commands.insert_resource(LocalCombatData {
        unit_entities: Default::default(),
    });
}

pub fn on_player_turn_to_place_unit(
    mut event: EventReader<PlayerTurnToPlaceUnit>,
    mut next_combat_state: ResMut<NextState<CombatState>>,
    mut combat_data: ResMut<CombatData>,
    local_player_id: Res<LocalPlayerId>,
) {
    for event in event.read() {
        if event.player == local_player_id.id {
            next_combat_state.set(CombatState::PlaceUnit);
            combat_data.current_turn = CombatTurn::place_unit(event.player);
        } else {
            next_combat_state.set(CombatState::WaitingForOtherPlayer);
            combat_data.current_turn = CombatTurn::place_unit(event.player);
        }
    }
}

pub fn on_start_unit_turn(
    mut commands: Commands,
    mut event: EventReader<StartUnitTurn>,
    mut next_combat_state: ResMut<NextState<CombatState>>,
    mut combat_data: ResMut<CombatData>,
    local_player_id: Res<LocalPlayerId>,
) {
    for event in event.read() {
        let locally_computed_next = combat_data.get_unit_for_next_turn();
        if locally_computed_next != event.unit_id {
            error!("Turn order seems to be out of sync? Server said to start turn for unit {}, but locally, it would be {}'s turn.",
                event.unit_id, locally_computed_next
            );
        }

        let Some(unit) = combat_data.units.get(&event.unit_id) else {
            error!("Was unable to find unit with ID {}.", event.unit_id);
            continue;
        };

        if unit.owner == local_player_id.actor {
            next_combat_state.set(CombatState::ThisPlayerUnitTurn);
        } else {
            next_combat_state.set(CombatState::WaitingForOtherPlayer)
        }

        commands.insert_resource(ActiveUnitHighlights {
            tile: unit.position,
        });
        combat_data.start_unit_turn(event.unit_id);
    }
}

pub fn on_combat_finished(
    mut events: EventReader<CombatFinished>,
    mut player_resources: ResMut<PlayerResources>,
    mut next_combat_state: ResMut<NextState<CombatState>>,
    local_player_id: Res<LocalPlayerId>,
) {
    for event in events.read() {
        // TODO: Persist lost units in Victory/Defeat state for UI
        player_resources
            .units
            .retain(|x| !event.casualties.contains(&x.id));

        for x in &mut player_resources.units {
            x.add_experience(event.experience);
        }

        if event.winners.contains(&local_player_id.actor) {
            next_combat_state.set(CombatState::Victory);
        } else {
            next_combat_state.set(CombatState::Defeated);
        }
    }
}
