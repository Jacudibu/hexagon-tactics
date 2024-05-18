use crate::game_data::base_stats::BaseStats;
use crate::game_data::skill::SkillId;

pub type ClassId = u32;

pub struct ClassDefinition {
    pub id: ClassId,
    pub name: String,
    pub stats_per_level: BaseStats, // Should we use a constant growth, randomize it a bit (+/- X) or follow a curve?
    pub learnable_skills: Vec<SkillId>,
}
