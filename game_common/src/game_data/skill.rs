use crate::combat_unit::{CombatUnit, UnitId};
use crate::game_map::GameMap;
use bevy::utils::HashMap;
use hexx::Hex;
use serde::{Deserialize, Serialize};

pub type SkillId = u32;

pub const DEBUG_SINGLE_TARGET_ATTACK_ID: SkillId = 1;
pub const DEBUG_AOE_TARGET_ATTACK_ID: SkillId = 2;
pub const DEBUG_AOE_T_SHAPED: SkillId = 3;

///
#[derive(Debug, Clone)]
pub struct SkillDefinition {
    pub id: SkillId,
    pub name: String,
    pub base_power: u32,
    pub mp_costs: u32,
    pub targeting: SkillTargeting,
    pub shape: SkillShape,
}

impl SkillDefinition {
    pub fn calculate_damage(
        &self,
        user: &CombatUnit,
        target: &CombatUnit,
    ) -> SkillInvocationResult {
        SkillInvocationResult {
            physical_damage: user.stats_after_buffs.strength + self.base_power,
            target_unit_id: target.id,
        }
    }

    pub fn get_valid_target_hexagons(
        &self,
        cursor_position: Hex,
        user_position: Hex,
        map: &GameMap,
    ) -> Vec<Hex> {
        let target_position = match self.targeting {
            SkillTargeting::UserPosition => user_position,
            SkillTargeting::MouseCursor(_) => cursor_position,
        };

        match &self.shape {
            SkillShape::SingleTile => {
                vec![cursor_position]
            }
            SkillShape::Circle(circle) => cursor_position
                .range(circle.radius)
                .filter(|hex| Self::is_tile_valid_for_map(hex, map))
                .collect(),
            SkillShape::Custom(custom) => {
                // Assuming EDGE_DIRECTION::POINTY_EAST is 0, so all shapes need to be aligned to the right.
                let rotations_needed = if custom.can_rotate {
                    if user_position == cursor_position {
                        0
                    } else {
                        user_position.main_direction_to(cursor_position).index() as u32
                    }
                } else {
                    0
                };

                custom
                    .tiles
                    .iter()
                    .map(|x| x.rotate_cw(rotations_needed) + target_position)
                    .filter(|hex| Self::is_tile_valid_for_map(hex, map))
                    .collect()
            }
        }
    }

    fn is_tile_valid_for_map(hex: &Hex, map: &GameMap) -> bool {
        let Some(tile) = map.tiles.get(hex) else {
            return false;
        };

        tile.height > 0
    }

    pub(in crate::game_data) fn mock_data() -> HashMap<SkillId, SkillDefinition> {
        let mut result = HashMap::new();

        result.insert(
            DEBUG_SINGLE_TARGET_ATTACK_ID,
            Self::debug_attack_single_target(),
        );
        result.insert(DEBUG_AOE_T_SHAPED, Self::debug_attack_t_shaped());
        result.insert(DEBUG_AOE_TARGET_ATTACK_ID, Self::debug_attack_aoe());

        result
    }

    fn debug_attack_single_target() -> SkillDefinition {
        SkillDefinition {
            id: DEBUG_SINGLE_TARGET_ATTACK_ID,
            name: "Single Target Debug Attack".into(),
            base_power: 5,
            mp_costs: 0,
            targeting: SkillTargeting::MouseCursor(SkillRange { min: 1, max: 1 }),
            shape: SkillShape::SingleTile,
        }
    }

    fn debug_attack_aoe() -> SkillDefinition {
        SkillDefinition {
            id: DEBUG_AOE_TARGET_ATTACK_ID,
            name: "Circular AoE Debug Attack".into(),
            base_power: 5,
            mp_costs: 0,
            targeting: SkillTargeting::MouseCursor(SkillRange { min: 2, max: 5 }),
            shape: SkillShape::Circle(CircleShapeData { radius: 1 }),
        }
    }

    fn debug_attack_t_shaped() -> SkillDefinition {
        SkillDefinition {
            id: DEBUG_AOE_T_SHAPED,
            name: "T-Shaped AoE Debug Attack".into(),
            base_power: 5,
            mp_costs: 0,
            targeting: SkillTargeting::UserPosition,
            shape: SkillShape::Custom(CustomShapeData {
                can_rotate: true,
                tiles: vec![
                    Hex::new(1, 0),
                    Hex::new(2, 0),
                    Hex::new(3, 0),
                    Hex::new(2, 1),
                    Hex::new(3, -1),
                ],
            }),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
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
    SingleTile,
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
    /// Whether the shape will be rotated to "face" the cursor position.
    pub can_rotate: bool,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SkillTargeting {
    UserPosition,
    MouseCursor(SkillRange),
}
