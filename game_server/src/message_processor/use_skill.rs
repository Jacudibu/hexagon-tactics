use crate::message_processor::ServerToClientMessageVariant;
use crate::state::MatchData;
use game_common::network_events::server_to_client::ServerToClientMessage;
use game_common::network_events::{client_to_server, server_to_client};
use game_common::player::PlayerId;
use game_common::validation;

pub fn use_skill(
    sender: PlayerId,
    match_data: &mut MatchData,
    data: client_to_server::UseSkill,
) -> Result<Vec<ServerToClientMessageVariant>, ServerToClientMessage> {
    validation::validate_turn_order(sender, &match_data.combat_data)?;
    todo!();
}

#[cfg(test)]
mod tests {
    use crate::message_processor::use_skill::use_skill;
    use crate::state::MatchData;
    use game_common::combat_data::CombatData;
    use game_common::game_map::GameMap;
    use game_common::network_events::client_to_server;
    use game_common::unit::Unit;
    use game_common::unit_stats::UnitStats;

    #[test]
    fn using_skill_should_work() {
        let user_id = 1;
        let target_id = 2;
        let skill_id = 1;
        let mut match_data = MatchData {
            combat_data: CombatData::create_mock()
                .with_units(vec![
                    Unit::create_mock(user_id, 1).with_stats(UnitStats::create_mock()),
                    Unit::create_mock(target_id, 2).with_stats(UnitStats::create_mock()),
                ])
                .with_unit_turn(user_id),
            loaded_map: GameMap::new(2),
        };

        let data = client_to_server::UseSkill { id: skill_id };

        let result = use_skill(1, &mut match_data, data).unwrap();
        assert_eq!(1, result.len());

        let result = result[0].as_broadcast().unwrap().as_use_skill().unwrap();
        assert_eq!(skill_id, result.id);

        let result = &result.result;
        assert_eq!(None, result.user, "Default skill should not affect user");

        let result = &result.target;
        assert_ne!(
            0, result.physical_damage,
            "Default skill should deal some damage"
        );

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
