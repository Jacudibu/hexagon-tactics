use crate::combat_data::CombatData;
use crate::combat_turn::CombatTurn;
use crate::game_map::GameMap;
use crate::validation::validation_error::ValidationError;
use hexx::Hex;

pub fn validate_path_for_current_unit(
    map: &GameMap,
    combat_data: &CombatData,
    path: &Vec<Hex>,
) -> Result<(), ValidationError> {
    if path.is_empty() {
        return Err(ValidationError::new("Path is empty!"));
    }

    let (unit, turn) = match &combat_data.current_turn {
        CombatTurn::Undefined => {
            return Err(ValidationError::new(
                "Path Validation called in undefined state!",
            ));
        }
        CombatTurn::PlaceUnit(_) => {
            return Err(ValidationError::new(
                "Path Validation called in undefined state!",
            ));
        }
        CombatTurn::UnitTurn(turn) => (&combat_data.units[&turn.unit_id], turn),
    };

    let current = path[0];
    if unit.position != current {
        return Err(ValidationError::new(
            "Unit position and path start don't match!",
        ));
    }

    if path.len() > (turn.remaining_movement as usize) + 1 {
        return Err(ValidationError::new("Not enough movement remaining!"));
    }

    let mut total_cost = 0;
    for i in 1..path.len() {
        match map.calculate_path_costs(unit, combat_data, &path[i - 1], &path[i]) {
            None => {
                return Err(ValidationError::new(format!(
                    "Unit cannot move from {:?} to {:?}",
                    path[i - 1],
                    path[i]
                )));
            }
            Some(cost) => total_cost += cost,
        };
    }

    if total_cost > (turn.remaining_movement as u32) {
        return Err(ValidationError::new("Not enough movement remaining!"));
    }

    Ok(())
}
