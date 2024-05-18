use crate::game_data::base_stats::BaseStats;
use crate::game_data::skill;
use crate::game_data::skill::SkillId;
use bevy::utils::HashMap;

pub struct EquipmentDefinition<T> {
    pub id: T,
    pub name: String,
    pub description: String,
    pub stats: BaseStats,
    pub skills: Vec<SkillId>,
}

pub type AccessoryId = usize;
pub type ArmorId = usize;
pub type WeaponId = usize;

pub type AccessoryDefinition = EquipmentDefinition<AccessoryId>;
pub type ArmorDefinition = EquipmentDefinition<ArmorId>;
pub type WeaponDefinition = EquipmentDefinition<WeaponId>;

pub const DEBUG_ACCESSORY_GARLIC_ID: AccessoryId = 1;
impl AccessoryDefinition {
    pub(in crate::game_data) fn mock_accessories() -> HashMap<AccessoryId, AccessoryDefinition> {
        let mut result = HashMap::new();

        result.insert(
            DEBUG_ACCESSORY_GARLIC_ID,
            WeaponDefinition {
                id: DEBUG_ACCESSORY_GARLIC_ID,
                name: "Debug Accessory: Garlic".into(),
                description: "It's quite stinky!".into(),
                stats: BaseStats {
                    movement: 0,
                    jump: 0,
                    strength: 0,
                    speed: 0,
                },
                skills: vec![skill::DEBUG_AOE_T_SHAPED],
            },
        );

        result
    }
}

pub const DEBUG_ARMOR_ID: ArmorId = 1;
impl ArmorDefinition {
    pub(in crate::game_data) fn mock_armor() -> HashMap<ArmorId, ArmorDefinition> {
        let mut result = HashMap::new();

        result.insert(
            DEBUG_ARMOR_ID,
            WeaponDefinition {
                id: DEBUG_ARMOR_ID,
                name: "Debug Armor".into(),
                description: "Important Plot Armor!".into(),
                stats: BaseStats {
                    movement: 0,
                    jump: 0,
                    strength: 0,
                    speed: 0,
                },
                skills: vec![],
            },
        );

        result
    }
}

pub const DEBUG_SWORD_ID: WeaponId = 1;
pub const DEBUG_STAFF_ID: WeaponId = 2;
impl WeaponDefinition {
    pub(in crate::game_data) fn mock_weapons() -> HashMap<WeaponId, WeaponDefinition> {
        let mut result = HashMap::new();

        result.insert(
            DEBUG_SWORD_ID,
            WeaponDefinition {
                id: DEBUG_SWORD_ID,
                name: "Debug Sword".into(),
                description: "Hit them with the pointy end!".into(),
                stats: BaseStats {
                    movement: 0,
                    jump: 0,
                    strength: 5,
                    speed: 0,
                },
                skills: vec![skill::DEBUG_SINGLE_TARGET_ATTACK_ID],
            },
        );
        result.insert(
            DEBUG_STAFF_ID,
            WeaponDefinition {
                id: DEBUG_STAFF_ID,
                name: "Debug Staff".into(),
                description: "Staff goes bonk!".into(),
                stats: BaseStats {
                    movement: 0,
                    jump: 0,
                    strength: 0,
                    speed: 0,
                },
                skills: vec![skill::DEBUG_AOE_TARGET_ATTACK_ID],
            },
        );

        result
    }
}
