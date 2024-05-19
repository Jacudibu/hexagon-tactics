use crate::game_data::base_stats::BaseStats;
use crate::game_data::skill::SkillId;
use bevy::utils::HashMap;

pub type ClassId = usize;
pub struct ClassDefinition {
    pub id: ClassId,
    pub name: String,
    pub stats_per_level: BaseStats, // Should we use a constant growth, randomize it a bit (+/- X) or follow a curve?
    pub learnable_skills: Vec<SkillId>,
}

pub const DEBUG_CLASS_FIGHTER: ClassId = 1;
pub const DEBUG_CLASS_MAGE: ClassId = 2;

impl ClassDefinition {
    pub fn mock_data() -> HashMap<ClassId, ClassDefinition> {
        let mut result = HashMap::new();

        result.insert(
            DEBUG_CLASS_FIGHTER,
            ClassDefinition {
                id: DEBUG_CLASS_FIGHTER,
                name: "Fighter".to_string(),
                stats_per_level: BaseStats {
                    hp: 5,
                    mp: 0,
                    movement: 0,
                    jump: 0,
                    strength: 5,
                    speed: 3,
                },
                learnable_skills: vec![],
            },
        );

        result.insert(
            DEBUG_CLASS_MAGE,
            ClassDefinition {
                id: DEBUG_CLASS_MAGE,
                name: "Mage".to_string(),
                stats_per_level: BaseStats {
                    hp: 0,
                    mp: 5,
                    movement: 0,
                    jump: 0,
                    strength: 0,
                    speed: 2,
                },
                learnable_skills: vec![],
            },
        );

        result
    }
}
