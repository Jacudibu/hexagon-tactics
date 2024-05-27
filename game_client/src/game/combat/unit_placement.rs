use crate::game::combat::combat_input::CombatAction;
use crate::game::combat::combat_plugin::CombatState;
use crate::game::combat::local_combat_data::LocalCombatData;
use crate::game::sprite_builder;
use crate::load::CharacterSprites;
use crate::map::{CursorOnTile, RangeHighlights};
use crate::networking::LocalPlayerId;
use bevy::prelude::*;
use bevy_sprite3d::Sprite3dParams;
use game_common::combat_data::CombatData;
use game_common::combat_unit::CombatUnit;
use game_common::game_data::unit_definition::UnitDefinition;
use game_common::game_map::GameMap;
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::{client_to_server, server_to_client};
use game_common::player_resources::PlayerResources;
use leafwing_input_manager::action_state::ActionState;

pub struct UnitPlacementPlugin;
impl Plugin for UnitPlacementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SwitchToNextUnitEvent>();
        app.add_systems(OnEnter(CombatState::PlaceUnit), setup_state);
        app.add_systems(OnExit(CombatState::PlaceUnit), leave_state);
        app.add_systems(
            Update,
            (
                on_switch_to_next_unit.run_if(on_event::<SwitchToNextUnitEvent>()),
                on_server_placed_unit.run_if(on_event::<server_to_client::PlaceUnit>()),
                input_listener.run_if(in_state(CombatState::PlaceUnit)),
            ),
        );
    }
}

#[derive(Resource, Debug)]
pub struct CurrentlyPlacedUnit {
    pub array_index: usize,
}

fn setup_state(
    mut commands: Commands,
    player_resources: Res<PlayerResources>,
    map: Res<GameMap>,
    combat_data: Res<CombatData>,
    local_player_id: Res<LocalPlayerId>,
) {
    let units = &player_resources.units;
    assert!(
        combat_data
            .units
            .iter()
            .filter(|(_, unit)| unit.owner == local_player_id.actor)
            .count()
            < units.len(),
        "All units have been placed, yet we just entered SetupState for CombatState::PlaceUnit ?",
    );

    commands.insert_resource(CurrentlyPlacedUnit {
        array_index: get_next_unplaced_unit_index(&combat_data, units, units.len()),
    });

    let spawn_spots = map.get_spawn_points_for_team(&1);
    commands.insert_resource(RangeHighlights { tiles: spawn_spots })
}

fn leave_state(mut commands: Commands) {
    commands.remove_resource::<CurrentlyPlacedUnit>();
    commands.remove_resource::<RangeHighlights>();
}

fn get_next_unplaced_unit_index(
    combat_data: &CombatData,
    units: &Vec<UnitDefinition>,
    index: usize,
) -> usize {
    fn increase_index(index: usize, unit_count: usize) -> usize {
        if index + 1 >= unit_count {
            0
        } else {
            index + 1
        }
    }

    let mut result = increase_index(index, units.len());
    while is_unit_already_placed(combat_data, units, result) {
        result = increase_index(result, units.len());
    }

    return result;
}

fn get_previous_unplaced_unit_index(
    combat_data: &CombatData,
    units: &Vec<UnitDefinition>,
    index: usize,
) -> usize {
    fn decrease_index(index: usize, unit_count: usize) -> usize {
        if index == 0 {
            unit_count - 1
        } else {
            index - 1
        }
    }

    let mut result = decrease_index(index, units.len());
    while is_unit_already_placed(combat_data, units, result) {
        result = decrease_index(result, units.len());
    }

    return result;
}

fn is_unit_already_placed(
    combat_data: &CombatData,
    units: &Vec<UnitDefinition>,
    index: usize,
) -> bool {
    let unit = &units[index];
    return combat_data.units.contains_key(&unit.id);
}

#[derive(Event)]
pub enum SwitchToNextUnitEvent {
    Previous,
    Next,
}

fn on_switch_to_next_unit(
    combat_data: Res<CombatData>,
    player_resources: Res<PlayerResources>,
    mut events: EventReader<SwitchToNextUnitEvent>,
    mut currently_placed_unit: ResMut<CurrentlyPlacedUnit>,
) {
    for event in events.read() {
        match event {
            SwitchToNextUnitEvent::Previous => {
                currently_placed_unit.array_index = get_previous_unplaced_unit_index(
                    &combat_data,
                    &player_resources.units,
                    currently_placed_unit.array_index,
                );
            }
            SwitchToNextUnitEvent::Next => {
                currently_placed_unit.array_index = get_next_unplaced_unit_index(
                    &combat_data,
                    &player_resources.units,
                    currently_placed_unit.array_index,
                );
            }
        }
    }
}

fn input_listener(
    player_resources: Res<PlayerResources>,
    action_state: Res<ActionState<CombatAction>>,
    cursor: Option<Res<CursorOnTile>>,
    currently_placed_unit: Res<CurrentlyPlacedUnit>,
    mut switch_unit_events: EventWriter<SwitchToNextUnitEvent>,
    mut client_to_server_events: EventWriter<ClientToServerMessage>,
) {
    let units = &player_resources.units;
    if action_state.just_pressed(&CombatAction::NextUnit) {
        switch_unit_events.send(SwitchToNextUnitEvent::Next);
    } else if action_state.just_pressed(&CombatAction::PreviousUnit) {
        switch_unit_events.send(SwitchToNextUnitEvent::Previous);
    } else if action_state.just_pressed(&CombatAction::SelectTile) {
        if let Some(cursor) = cursor {
            // TODO: Validation so we can't spam the server
            client_to_server_events.send(ClientToServerMessage::PlaceUnit(
                client_to_server::PlaceUnit {
                    unit_id: units[currently_placed_unit.array_index].id,
                    hex: cursor.hex,
                },
            ));
        }
    }
}

fn on_server_placed_unit(
    mut commands: Commands,
    character_sprites: Res<CharacterSprites>,
    map: Res<GameMap>,
    mut sprite_params: Sprite3dParams,
    mut events: EventReader<server_to_client::PlaceUnit>,
    mut combat_data: ResMut<CombatData>,
    mut local_combat_data: ResMut<LocalCombatData>,
    mut next_state: ResMut<NextState<CombatState>>,
) {
    for event in events.read() {
        let unit = event.unit.clone();

        let entity = spawn_unit_entity(
            &mut commands,
            &character_sprites,
            &map,
            &mut sprite_params,
            &unit,
        );

        local_combat_data.unit_entities.insert(unit.id, entity);

        combat_data.unit_positions.insert(unit.position, unit.id);
        combat_data.units.insert(unit.id, unit);

        next_state.set(CombatState::WaitingForServer)
    }
}

#[derive(Component)]
pub struct UnitMarker;

fn spawn_unit_entity(
    commands: &mut Commands,
    character_sprites: &CharacterSprites,
    map: &GameMap,
    sprite_params: &mut Sprite3dParams,
    unit: &CombatUnit,
) -> Entity {
    commands
        .spawn((
            Name::new(unit.name.clone()),
            UnitMarker {},
            sprite_builder::build_unit_sprite(unit, character_sprites, map, sprite_params),
        ))
        .id()
}
