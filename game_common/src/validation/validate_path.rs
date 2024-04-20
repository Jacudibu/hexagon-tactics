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

#[cfg(test)]
mod tests {
    use crate::combat_data::CombatData;
    use crate::game_map::GameMap;
    use crate::unit::Unit;
    use crate::unit_stats::UnitStats;
    use crate::validation::validate_path_for_current_unit;
    use hexx::Hex;

    #[test]
    fn valid_path() {
        let unit_pos = Hex::new(-2, 0);
        let target_pos = Hex::new(2, 0);
        let map = GameMap::new(2);
        let combat_data = CombatData::create_mock()
            .with_units(vec![Unit::create_mock(1, 1)
                .with_position(unit_pos)
                .with_stats(UnitStats::create_mock().with_movement(4))])
            .with_unit_turn(1);
        let path = hexx::algorithms::a_star(unit_pos, target_pos, |_, _| Some(1)).unwrap();

        let result = validate_path_for_current_unit(&map, &combat_data, &path);
        assert!(result.is_ok(), "{:?}", result);
    }

    #[test]
    fn empty_path() {
        let map = GameMap::new(1);
        let combat_data = CombatData::create_mock();
        let result = validate_path_for_current_unit(&map, &combat_data, &Vec::new());
        assert!(result.is_err());
    }

    #[test]
    fn path_too_long() {
        let unit_pos = Hex::new(-2, 0);
        let target_pos = Hex::new(2, 0);
        let map = GameMap::new(2);
        let combat_data = CombatData::create_mock()
            .with_units(vec![Unit::create_mock(1, 1)
                .with_position(unit_pos)
                .with_stats(UnitStats::create_mock().with_movement(3))])
            .with_unit_turn(1);
        let path = hexx::algorithms::a_star(unit_pos, target_pos, |_, _| Some(1)).unwrap();

        let result = validate_path_for_current_unit(&map, &combat_data, &path);
        assert!(result.is_err());
    }
}
