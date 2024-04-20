use crate::combat_data::CombatData;
use crate::combat_turn::CombatTurn;
use crate::player::PlayerId;
use crate::unit::{Unit, UnitId};
use crate::validation::validation_error::ValidationError;

pub fn validate_player_owns_unit_with_id(
    player_id: PlayerId,
    unit_id: UnitId,
    combat_data: &CombatData,
) -> Result<(), ValidationError> {
    match &combat_data.current_turn {
        CombatTurn::Undefined => Err(ValidationError::new("Undefined turn behaviour!")),
        CombatTurn::PlaceUnit(_) => validate_player_owns_unit_option(
            player_id,
            combat_data.unit_storage.iter().find(|x| x.id == unit_id),
        ),
        CombatTurn::UnitTurn(_) => {
            validate_player_owns_unit_option(player_id, combat_data.units.get(&unit_id))
        }
    }
}

fn validate_player_owns_unit_option(
    player_id: PlayerId,
    unit: Option<&Unit>,
) -> Result<(), ValidationError> {
    match unit {
        None => Err(ValidationError::new("Unable to find unit!")),
        Some(unit) => {
            if unit.owner == player_id {
                Ok(())
            } else {
                Err(ValidationError::new("You do not own that unit!"))
            }
        }
    }
}
