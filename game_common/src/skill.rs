use crate::game_map::GameMap;
use crate::unit::{Unit, UnitId};
use hexx::Hex;
use serde::{Deserialize, Serialize};

pub type SkillId = u32;

pub const DEBUG_SINGLE_TARGET_ATTACK_ID: SkillId = 1;
pub const DEBUG_AOE_TARGET_ATTACK_ID: SkillId = 2;
pub const DEBUG_AOE_T_SHAPED: SkillId = 3;

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
            &DEBUG_AOE_T_SHAPED => Skill::debug_attack_t_shaped(),
            _ => {
                todo!()
            }
        }
    }

    pub fn get_valid_target_hexagons(
        &self,
        cursor_position: Hex,
        user_position: Hex,
        map: &GameMap,
    ) -> Vec<Hex> {
        match &self.shape {
            SkillShape::CursorTile => {
                vec![cursor_position]
            }
            SkillShape::Circle(circle) => cursor_position
                .range(circle.radius)
                .filter(|x| {
                    let Some(tile) = map.tiles.get(x) else {
                        return false;
                    };

                    tile.height > 0
                })
                .collect(),
            SkillShape::Custom(custom) => {
                let target_pos = if custom.centered_around_user {
                    user_position
                } else {
                    cursor_position
                };

                // Assuming EDGE_DIRECTION::POINTY_EAST is 0, so all shapes need to be aligned to the right.
                let rotations_needed =
                    user_position.main_direction_to(cursor_position).index() as u32;

                custom
                    .tiles
                    .iter()
                    .map(|x| x.rotate_cw(rotations_needed) + target_pos)
                    .filter(|x| {
                        let Some(tile) = map.tiles.get(x) else {
                            return false;
                        };

                        tile.height > 0
                    })
                    .collect()
            }
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

    fn debug_attack_t_shaped() -> Skill {
        Skill {
            id: DEBUG_AOE_T_SHAPED,
            name: "Debug Attack".into(),
            base_power: 5,
            mp_costs: 0,
            range: SkillRange { min: 1, max: 1 },
            shape: SkillShape::Custom(CustomShapeData {
                tiles: vec![
                    Hex::new(1, 0),
                    Hex::new(2, 0),
                    Hex::new(3, 0),
                    Hex::new(2, 1),
                    Hex::new(3, -1),
                ],
                centered_around_user: true,
                rotate: true,
            }),
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
    Custom(CustomShapeData),
}

#[derive(Debug, Clone)]
pub struct CircleShapeData {
    pub radius: u32,
}

#[derive(Debug, Clone)]
pub struct CustomShapeData {
    pub tiles: Vec<Hex>,
    pub centered_around_user: bool,
    pub rotate: bool,
}
