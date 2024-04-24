use crate::unit::{Unit, UnitId};
use serde::{Deserialize, Serialize};

pub type SkillId = u32;

pub const DEBUG_ATTACK_ID: SkillId = 1;

#[derive(Debug, Clone)]
pub struct Skill {
    pub id: SkillId,
    pub name: String,
    pub base_power: u32,
    pub mp_costs: u32,
    pub range: SkillRange,
}

impl Skill {
    pub fn calculate_damage(&self, user: &Unit, target: &Unit) -> SkillInvocationResult {
        SkillInvocationResult {
            physical_damage: user.stats_after_buffs.strength + self.base_power,
            target_unit_id: target.id,
        }
    }

    pub fn get(id: &SkillId) -> Skill {
        match id {
            1 => Skill::debug_attack(),
            _ => {
                todo!()
            }
        }
    }

    pub fn debug_attack() -> Skill {
        Skill {
            id: DEBUG_ATTACK_ID,
            name: "Debug Attack".into(),
            base_power: 5,
            mp_costs: 0,
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
    pub target_unit_id: UnitId,
    pub physical_damage: u32,
}
