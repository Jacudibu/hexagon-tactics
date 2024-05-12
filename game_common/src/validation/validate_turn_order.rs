use crate::combat_data::CombatData;
use crate::combat_turn::CombatTurn;
use crate::combat_unit::Owner;
use crate::player::PlayerId;
use crate::validation::validation_error::ValidationError;

pub fn validate_turn_order(
    player_id: PlayerId,
    combat_data: &CombatData,
) -> Result<(), ValidationError> {
    match &combat_data.current_turn {
        CombatTurn::Undefined => Err(ValidationError::new("Undefined turn behaviour!")),
        CombatTurn::PlaceUnit(turn) => {
            if turn.player_id == player_id {
                Ok(())
            } else {
                Err(ValidationError::new("It's not your turn!"))
            }
        }
        CombatTurn::UnitTurn(turn) => {
            let unit = &combat_data.units[&turn.unit_id];

            if unit.owner == Owner::Player(player_id) {
                Ok(())
            } else {
                Err(ValidationError::new("It's not your turn!"))
            }
        }
    }
}
