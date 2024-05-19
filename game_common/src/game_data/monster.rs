use crate::game_data::base_stats::BaseStats;
use crate::game_data::skill::SkillId;
use bevy::utils::HashMap;

pub type MonsterId = u32;

pub struct MonsterDefinition {
    pub id: MonsterId,
    pub name: String,
    pub stats: BaseStats,
    pub skills: Vec<SkillId>,
}

pub const DEBUG_MONSTER_ID: MonsterId = 1;

impl MonsterDefinition {
    pub(in crate::game_data) fn mock_data() -> HashMap<MonsterId, MonsterDefinition> {
        let mut result = HashMap::new();

        result.insert(
            DEBUG_MONSTER_ID,
            Self {
                id: DEBUG_MONSTER_ID,
                name: "Slime".into(),
                skills: Vec::new(),
                stats: BaseStats {
                    hp: 5,
                    mp: 5,
                    movement: 3,
                    jump: 3,
                    strength: 5,
                    speed: 40,
                },
            },
        );

        result
    }
}
