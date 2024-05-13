use crate::combat_data::CombatData;
use crate::combat_turn::CombatTurn;
use crate::combat_unit::{ActorId, CombatUnit, UnitId};
use crate::game_data::unit_definition::UnitDefinition;
use crate::player::PlayerId;
use crate::player_resources::PlayerResources;
use crate::validation::validation_error::ValidationError;
use bevy::utils::hashbrown::HashMap;

pub fn validate_player_owns_combat_unit_with_id<'a>(
    player_id: PlayerId,
    unit_id: UnitId,
    combat_data: &CombatData,
) -> Result<&CombatUnit, ValidationError> {
    match &combat_data.current_turn {
        CombatTurn::Undefined => Err(ValidationError::new("Undefined turn behaviour!")),
        CombatTurn::PlaceUnit(_) => Err(ValidationError::new("Undefined turn behaviour!")),
        CombatTurn::UnitTurn(_) => {
            validate_player_owns_unit_option(player_id, combat_data.units.get(&unit_id))
        }
    }
}

pub fn validate_player_owns_resource_unit_with_id(
    player_id: PlayerId,
    unit_id: UnitId,
    player_resources: &HashMap<PlayerId, PlayerResources>,
) -> Result<&UnitDefinition, ValidationError> {
    let units = &player_resources[&player_id].units;
    match units.iter().find(|x| x.id == unit_id) {
        None => Err(ValidationError::new("You do not own that unit!")),
        Some(unit) => Ok(unit),
    }
}

fn validate_player_owns_unit_option(
    player_id: PlayerId,
    unit: Option<&CombatUnit>,
) -> Result<&CombatUnit, ValidationError> {
    match unit {
        None => Err(ValidationError::new("Unable to find unit!")),
        Some(unit) => {
            if unit.owner == ActorId::Player(player_id) {
                Ok(unit)
            } else {
                Err(ValidationError::new("You do not own that unit!"))
            }
        }
    }
}
