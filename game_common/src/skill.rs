use crate::game_map::GameMap;
use crate::unit::Unit;
use serde::{Deserialize, Serialize};

pub type SkillId = u32;

#[derive(Debug, Clone)]
pub struct Skill {
    pub id: SkillId,
    pub name: String,
    pub base_power: u32,
    pub range: SkillRange,
}

impl Skill {
    pub fn calculate_damage(user: &Unit, target: &Unit, map: &GameMap) -> SkillInvocationResult {
        SkillInvocationResult::default()
    }

    pub fn debug_attack() -> Skill {
        Skill {
            id: 1,
            name: "Debug Attack".into(),
            base_power: 5,
            range: SkillRange { min: 1, max: 1 },
        }
    }
}

#[derive(Debug, Clone)]
pub struct SkillRange {
    pub min: u32,
    pub max: u32,
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
pub struct SkillInvocationResult {
    pub target: SkillInvocationResultElement,
    pub user: Option<SkillInvocationResultElement>,
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
pub struct SkillInvocationResultElement {
    pub physical_damage: u32,
}
