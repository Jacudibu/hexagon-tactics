use crate::message_processor::validation::validation_error::ValidationError;
use game_common::combat_data::CombatData;
use game_common::combat_turn::CombatTurn;
use game_common::player::PlayerId;

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

            if unit.owner == player_id {
                Ok(())
            } else {
                Err(ValidationError::new("It's not your turn!"))
            }
        }
    }
}
