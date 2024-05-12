use crate::in_game_state::MatchData;
use crate::message_processor::ServerToClientMessageVariant;
use game_common::game_data::GameData;
use game_common::network_events::server_to_client::{
    ErrorWhenProcessingMessage, ServerToClientMessage,
};
use game_common::network_events::{client_to_server, server_to_client};
use game_common::player::PlayerId;
use game_common::validation;

pub fn use_skill(
    sender: PlayerId,
    message: client_to_server::UseSkill,
    match_data: &mut MatchData,
    game_data: &GameData,
) -> Result<Vec<ServerToClientMessageVariant>, ServerToClientMessage> {
    validation::validate_turn_order(sender, &match_data.combat_data)?;
    validation::validate_unit_has_at_least_one_action(&match_data.combat_data)?;

    let unit_id = match_data
        .combat_data
        .current_turn
        .as_unit_turn()
        .unwrap()
        .unit_id;

    let Some(used_skill) = game_data.skills.get(&message.id) else {
        return Err(ServerToClientMessage::ErrorWhenProcessingMessage(
            ErrorWhenProcessingMessage {
                message: format!("Invalid Skill Id: {}", message.id),
            },
        ));
    };
    let user = &match_data.combat_data.units[&unit_id];

    validation::validate_unit_has_enough_resources_to_use_skill(user, used_skill)?;
    validation::validate_skill_target_is_in_range(
        &used_skill,
        user.position,
        message.target_coordinates,
    )?;

    let mut hits = Vec::new();
    let targets = used_skill
        .get_valid_target_hexagons(
            message.target_coordinates,
            user.position,
            &match_data.loaded_map,
        )
        .into_iter()
        .filter_map(
            |hex| match match_data.combat_data.unit_positions.get(&hex) {
                None => None,
                Some(unit_id) => Some(unit_id),
            },
        );

    for unit_id in targets {
        let target = &match_data.combat_data.units[unit_id];
        let hit = used_skill.calculate_damage(user, target);
        hits.push(hit);
    }

    for x in &hits {
        let unit = match_data
            .combat_data
            .units
            .get_mut(&x.target_unit_id)
            .unwrap();

        if x.physical_damage > unit.hp {
            unit.hp = 0;
        } else {
            unit.hp -= x.physical_damage;
        }
    }

    match_data
        .combat_data
        .current_turn
        .as_unit_turn_mut()
        .unwrap()
        .remaining_actions -= 1;

    let user = match_data.combat_data.units.get_mut(&unit_id).unwrap();
    user.mp -= used_skill.mp_costs;

    return Ok(vec![ServerToClientMessageVariant::Broadcast(
        ServerToClientMessage::UseSkill(server_to_client::UseSkill {
            id: message.id,
            target_coordinates: message.target_coordinates,
            hits,
        }),
    )]);
}

#[cfg(test)]
mod tests {
    use crate::in_game_state::MatchData;
    use crate::message_processor::combat::use_skill::use_skill;
    use game_common::combat_data::CombatData;
    use game_common::combat_unit::CombatUnit;
    use game_common::game_data::GameData;
    use game_common::game_map::GameMap;
    use game_common::network_events::client_to_server;
    use game_common::unit_stats::UnitStats;
    use hexx::{EdgeDirection, Hex};

    #[test]
    fn using_skill_should_work() {
        let user_id = 1;
        let target_id = 2;
        let target_position = Hex::ZERO.neighbor(EdgeDirection::POINTY_RIGHT);
        let skill_id = 1;
        let game_data = GameData::create_mock().with_all_mock_skills();
        let mut match_data = MatchData {
            combat_data: CombatData::create_mock()
                .with_units(vec![
                    CombatUnit::create_mock(user_id, 1)
                        .with_position(Hex::ZERO)
                        .with_stats(UnitStats::create_mock()),
                    CombatUnit::create_mock(target_id, 2)
                        .with_position(target_position)
                        .with_stats(UnitStats::create_mock()),
                ])
                .with_unit_turn(user_id),
            loaded_map: GameMap::new(2),
        };

        let message = client_to_server::UseSkill {
            id: skill_id,
            target_coordinates: target_position,
        };

        let old_hp = match_data.combat_data.units[&target_id].hp;
        let result = use_skill(1, message, &mut match_data, &game_data).unwrap();
        assert_eq!(1, result.len());

        let result = result[0].as_broadcast().unwrap().as_use_skill().unwrap();
        assert_eq!(skill_id, result.id);
        assert_eq!(target_position, result.target_coordinates);

        assert_eq!(1, result.hits.len());
        let result = &result.hits[0];
        assert_ne!(
            0, result.physical_damage,
            "Default skill should deal some damage"
        );

        assert_eq!(
            old_hp - result.physical_damage,
            match_data.combat_data.units[&target_id].hp
        );

        // TODO: check resource consumption

        assert_eq!(
            match_data
                .combat_data
                .current_turn
                .as_unit_turn()
                .unwrap()
                .remaining_actions,
            0
        );
    }
}
