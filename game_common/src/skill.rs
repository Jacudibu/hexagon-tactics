use crate::game_map::GameMap;
use crate::unit::{Unit, UnitId};
use hexx::Hex;
use serde::{Deserialize, Serialize};

pub type SkillId = u32;

pub const DEBUG_SINGLE_TARGET_ATTACK_ID: SkillId = 1;
pub const DEBUG_AOE_TARGET_ATTACK_ID: SkillId = 2;

#[derive(Debug, Clone)]
pub struct Skill {
    pub id: SkillId,
    pub name: String,
    pub base_power: u32,
    pub mp_costs: u32,
    pub range: SkillRange,
    pub shape: SkillShape,
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
            &DEBUG_SINGLE_TARGET_ATTACK_ID => Skill::debug_attack_single_target(),
            &DEBUG_AOE_TARGET_ATTACK_ID => Skill::debug_attack_aoe(),
            _ => {
                todo!()
            }
        }
    }

    pub fn get_valid_target_hexagons(&self, center_coordinate: Hex, map: &GameMap) -> Vec<Hex> {
        match &self.shape {
            SkillShape::CursorTile => {
                vec![center_coordinate]
            }
            SkillShape::Circle(circle) => center_coordinate
                .range(circle.radius)
                .filter(|x| {
                    let Some(tile) = map.tiles.get(x) else {
                        return false;
                    };

                    tile.height > 0
                })
                .collect(),
        }
    }

    fn debug_attack_single_target() -> Skill {
        Skill {
            id: DEBUG_SINGLE_TARGET_ATTACK_ID,
            name: "Debug Attack".into(),
            base_power: 5,
            mp_costs: 0,
            range: SkillRange { min: 1, max: 1 },
            shape: SkillShape::CursorTile,
        }
    }

    fn debug_attack_aoe() -> Skill {
        Skill {
            id: DEBUG_AOE_TARGET_ATTACK_ID,
            name: "Debug Attack".into(),
            base_power: 5,
            mp_costs: 0,
            range: SkillRange { min: 2, max: 5 },
            shape: SkillShape::Circle(CircleShapeData { radius: 1 }),
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

#[derive(Debug, Clone)]
pub enum SkillShape {
    CursorTile,
    Circle(CircleShapeData),
}

#[derive(Debug, Clone)]
pub struct CircleShapeData {
    pub radius: u32,
}
