use crate::combat::combat_input::CombatAction;
use crate::combat::combat_plugin::CombatState;
use crate::combat::local_combat_data::LocalCombatData;
use crate::combat_data_resource::CombatDataResource;
use crate::load::CharacterSprites;
use crate::map::{CursorOnTile, METERS_PER_TILE_HEIGHT_UNIT};
use bevy::prelude::*;
use bevy_sprite3d::{Sprite3d, Sprite3dParams};
use game_common::game_map::GameMap;
use game_common::game_map::HEX_LAYOUT;
use game_common::network_events::client_to_server::ClientToServerMessage;
use game_common::network_events::{client_to_server, server_to_client};
use game_common::unit::Unit;
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
    pub array_index: usize,
}

fn setup_state(mut commands: Commands) {
    commands.insert_resource(CurrentlyPlacedUnit { array_index: 0 });
}

fn leave_state(mut commands: Commands) {
    commands.remove_resource::<CurrentlyPlacedUnit>();
}

fn input_listener(
    mut currently_placed_unit: ResMut<CurrentlyPlacedUnit>,
    action_state: Res<ActionState<CombatAction>>,
    combat_data: Res<CombatDataResource>,
    cursor: Option<Res<CursorOnTile>>,
    mut client_to_server_events: EventWriter<ClientToServerMessage>,
) {
    let units = &combat_data.unit_storage;
    if action_state.just_pressed(&CombatAction::NextUnit) {
        if currently_placed_unit.array_index + 1 >= units.len() {
            currently_placed_unit.array_index = 0;
        } else {
            currently_placed_unit.array_index += 1;
        }
    } else if action_state.just_pressed(&CombatAction::PreviousUnit) {
        if currently_placed_unit.array_index == 0 {
            currently_placed_unit.array_index = units.len() - 1;
        } else {
            currently_placed_unit.array_index -= 1;
        }
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
    mut combat_data: ResMut<CombatDataResource>,
    mut local_combat_data: ResMut<LocalCombatData>,
    mut next_state: ResMut<NextState<CombatState>>,
) {
    for event in events.read() {
        let Some(index) = combat_data
            .unit_storage
            .iter()
            .position(|x| x.id == event.unit_id)
        else {
            error!(
                "Was unable to find unit with id {} in unit storage!",
                event.unit_id
            );
            continue;
        };

        let mut unit = combat_data.unit_storage.remove(index);
        unit.position = event.hex;

        let entity = spawn_unit_entity(
            &mut commands,
            &character_sprites,
            &map,
            &mut sprite_params,
            &unit,
            event.hex,
        );

        local_combat_data.unit_entities.insert(unit.id, entity);

        combat_data.unit_positions.insert(event.hex, event.unit_id);
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
    mut sprite_params: &mut Sprite3dParams,
    unit: &Unit,
    hex: Hex,
) -> Entity {
    commands
        .spawn((
            Name::new(unit.name.clone()),
            UnitMarker {},
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
        ))
        .id()
}

pub fn unit_position_on_hexagon(hex: Hex, map: &GameMap) -> Vec3 {
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
