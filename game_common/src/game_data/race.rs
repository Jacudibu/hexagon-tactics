use crate::game_data::base_stats::BaseStats;
use crate::game_data::skill::SkillId;
use bevy::utils::HashMap;

pub type RaceId = usize;

pub struct RaceDefinition {
    pub id: RaceId,
    pub name: String,
    pub base_stats: BaseStats,
    pub extra_skills: Vec<SkillId>,
}

pub const DEBUG_RACE_ID: RaceId = 1;

impl RaceDefinition {
    pub(in crate::game_data) fn mock_data() -> HashMap<RaceId, RaceDefinition> {
        let mut result = HashMap::new();

        result.insert(
            DEBUG_RACE_ID,
            Self {
                id: DEBUG_RACE_ID,
                name: "DEBUG RACE".into(),
                extra_skills: Vec::new(),
                base_stats: BaseStats {
                    movement: 3,
                    jump: 3,
                    strength: 5,
                    speed: 50,
                },
            },
        );

        result
    }
}
