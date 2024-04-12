use crate::combat::combat_input::CombatAction;
use crate::combat::CombatState;
use crate::load::CharacterSprites;
use crate::map::{MouseCursorOnTile, METERS_PER_TILE_HEIGHT_UNIT};
use bevy::prelude::*;
use bevy_sprite3d::{Sprite3d, Sprite3dParams};
use game_common::game_map::{GameMap, TileData, HEX_LAYOUT};
use game_common::game_state::CombatData;
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::server_to_client::PlaceUnit;
use game_common::network_events::{client_to_server, server_to_client};
use game_common::units::UnitId;
use hexx::Hex;
use leafwing_input_manager::action_state::ActionState;

pub struct UnitPlacementPlugin;
impl Plugin for UnitPlacementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(CombatState::PlaceUnit), setup_state);
        app.add_systems(OnExit(CombatState::PlaceUnit), leave_state);
        app.add_systems(
            Update,
            (
                on_server_placed_unit.run_if(on_event::<server_to_client::PlaceUnit>()),
                input_listener.run_if(in_state(CombatState::PlaceUnit)),
            ),
        );
    }
}

#[derive(Resource, Debug)]
pub struct CurrentlyPlacedUnit {
    pub unit_id: UnitId, // Consider just storing the array_index
    array_index: usize,
}

fn setup_state(mut commands: Commands, combat_data: Res<CombatData>) {
    let unit_id = combat_data.units_that_can_still_be_placed.first().unwrap();
    commands.insert_resource(CurrentlyPlacedUnit {
        unit_id: unit_id.clone(),
        array_index: 0,
    });
}

fn leave_state(mut commands: Commands) {
    commands.remove_resource::<CurrentlyPlacedUnit>();
}

fn input_listener(
    mut currently_placed_unit: ResMut<CurrentlyPlacedUnit>,
    action_state: Res<ActionState<CombatAction>>,
    combat_data: Res<CombatData>,
    cursor: Option<Res<MouseCursorOnTile>>,
    mut client_to_server_events: EventWriter<ClientToServerMessage>,
) {
    let units = &combat_data.units_that_can_still_be_placed;
    if action_state.just_pressed(&CombatAction::NextUnit) {
        if currently_placed_unit.array_index + 1 >= units.len() {
            currently_placed_unit.array_index = 0;
        } else {
            currently_placed_unit.array_index += 1;
        }

        currently_placed_unit.unit_id = units[currently_placed_unit.array_index].clone();
    } else if action_state.just_pressed(&CombatAction::PreviousUnit) {
        if currently_placed_unit.array_index == 0 {
            currently_placed_unit.array_index = units.len() - 1;
        } else {
            currently_placed_unit.array_index -= 1;
        }

        currently_placed_unit.unit_id = units[currently_placed_unit.array_index].clone();
    } else if action_state.just_pressed(&CombatAction::SelectTile) {
        if let Some(cursor) = cursor {
            // TODO: Validation
            client_to_server_events.send(ClientToServerMessage::PlaceUnit(
                client_to_server::PlaceUnit {
                    unit_id: currently_placed_unit.unit_id,
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
    mut next_state: ResMut<NextState<CombatState>>,
) {
    for event in events.read() {
        combat_data
            .units_that_can_still_be_placed
            .retain(|x| x != &event.unit_id);
        combat_data.unit_positions.insert(event.hex, event.unit_id);

        spawn_unit_entity(
            &mut commands,
            &character_sprites,
            &map,
            &mut sprite_params,
            &mut combat_data,
            &event.unit_id,
            event.hex,
        );

        next_state.set(CombatState::WaitingForServer)
    }
}

fn spawn_unit_entity(
    commands: &mut Commands,
    character_sprites: &CharacterSprites,
    map: &GameMap,
    mut sprite_params: &mut Sprite3dParams,
    combat_data: &mut ResMut<CombatData>,
    id: &UnitId,
    hex: Hex,
) -> bool {
    let Some(unit) = combat_data.units.get(id) else {
        error!("Was unable to find unit with id {} in unit list!", id);
        return true;
    };

    commands.spawn((
        Name::new(unit.name.clone()),
        Sprite3d {
            image: character_sprites.test.clone(),
            pixels_per_metre: 16.0,
            alpha_mode: AlphaMode::Mask(0.1),
            unlit: false,
            double_sided: true, // required for shadows
            pivot: Some(Vec2::new(0.5, 0.0)),
            transform: Transform::from_translation(unit_position_on_hexagon(hex, &map)),
            ..default()
        }
        .bundle(&mut sprite_params),
    ));
    false
}

fn unit_position_on_hexagon(hex: Hex, map: &GameMap) -> Vec3 {
    let height = match map.tiles.get(&hex) {
        None => {
            error!(
                "Was unable to find tile for hex when solving unit position: {:?}",
                hex
            );
            0.0
        }
        Some(tile_data) => tile_data.height as f32 * METERS_PER_TILE_HEIGHT_UNIT,
    };

    let hex_pos = HEX_LAYOUT.hex_to_world_pos(hex);

    Vec3::new(hex_pos.x, height, hex_pos.y)
}
