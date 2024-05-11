use crate::in_game_state::MatchData;
use crate::message_processor::ServerToClientMessageVariant;
use game_common::network_events::server_to_client::ServerToClientMessage;
use game_common::network_events::{client_to_server, server_to_client};
use game_common::player::PlayerId;
use game_common::validation;

pub fn move_unit(
    sender: PlayerId,
    message: client_to_server::MoveUnit,
    match_data: &mut MatchData,
) -> Result<Vec<ServerToClientMessageVariant>, ServerToClientMessage> {
    validation::validate_turn_order(sender, &match_data.combat_data)?;
    validation::validate_path_for_current_unit(
        &match_data.loaded_map,
        &match_data.combat_data,
        &message.path,
    )?;

    let turn = match_data
        .combat_data
        .current_turn
        .as_unit_turn_mut()
        .unwrap();
    turn.remaining_movement -= message.path.len() as u8 - 1;

    let unit = match_data.combat_data.units.get_mut(&turn.unit_id).unwrap();

    match_data.combat_data.unit_positions.remove(&unit.position);
    unit.position = message.path.last().unwrap().clone();
    match_data
        .combat_data
        .unit_positions
        .insert(unit.position, unit.id);

    Ok(vec![ServerToClientMessageVariant::Broadcast(
        ServerToClientMessage::MoveUnit(server_to_client::MoveUnit { path: message.path }),
    )])
}

#[cfg(test)]
mod tests {
    use crate::in_game_state::MatchData;
    use crate::message_processor::combat::move_unit::move_unit;
    use game_common::combat_data::CombatData;
    use game_common::game_map::GameMap;
    use game_common::network_events::client_to_server::MoveUnit;
    use game_common::unit::Unit;
    use game_common::unit_stats::UnitStats;
    use hexx::{EdgeDirection, Hex};

    #[test]
    fn move_unit_with_valid_data_should_work() {
        let unit_id = 1;
        let unit_movement = 3;
        let unit_start_pos = Hex::ZERO;
        let unit_new_pos = unit_start_pos.neighbor(EdgeDirection::POINTY_RIGHT);
        let mut match_data = MatchData {
            combat_data: CombatData::create_mock()
                .with_units(vec![Unit::create_mock(unit_id, 1)
                    .with_stats(UnitStats::create_mock().with_movement(unit_movement))])
                .with_unit_turn(unit_id),
            loaded_map: GameMap::new(2),
        };

        let path = vec![unit_start_pos, unit_new_pos];
        let message = MoveUnit { path: path.clone() };

        let result = move_unit(1, message, &mut match_data).unwrap();
        assert_eq!(1, result.len());
        let result = result[0].as_broadcast().unwrap().as_move_unit().unwrap();
        assert_eq!(path, result.path);

        assert_eq!(
            match_data.combat_data.units[&unit_id].position,
            unit_new_pos
        );
        assert_eq!(
            match_data
                .combat_data
                .current_turn
                .as_unit_turn()
                .unwrap()
                .remaining_movement,
            unit_movement - 1
        );
    }
}
