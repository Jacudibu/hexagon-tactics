use crate::combat_data::CombatData;
use crate::combat_turn::CombatTurn;
use crate::validation::validation_error::ValidationError;

pub fn validate_unit_has_at_least_one_action(
    combat_data: &CombatData,
) -> Result<(), ValidationError> {
    if let CombatTurn::UnitTurn(turn) = &combat_data.current_turn {
        if turn.remaining_actions > 0 {
            Ok(())
        } else {
            Err(ValidationError::new(
                "Unit has no more actions for this turn!",
            ))
        }
    } else {
        Err(ValidationError::new("Undefined turn behaviour!"))
    }
}
