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
pub const DEBUG_ACCESSORY_BOOTS_ID: AccessoryId = 2;
impl AccessoryDefinition {
    pub(in crate::game_data) fn mock_accessories() -> HashMap<AccessoryId, AccessoryDefinition> {
        let mut result = HashMap::new();

        result.insert(
            DEBUG_ACCESSORY_GARLIC_ID,
            AccessoryDefinition {
                id: DEBUG_ACCESSORY_GARLIC_ID,
                name: "Garlic".into(),
                description: "It's quite stinky!".into(),
                stats: BaseStats {
                    hp: 2,
                    mp: 4,
                    movement: 0,
                    jump: 0,
                    strength: 0,
                    speed: 0,
                },
                skills: vec![skill::DEBUG_AOE_T_SHAPED],
            },
        );

        result.insert(
            DEBUG_ACCESSORY_BOOTS_ID,
            AccessoryDefinition {
                id: DEBUG_ACCESSORY_BOOTS_ID,
                name: "Reed Boots".into(),
                description: "Gotta go fast!!".into(),
                stats: BaseStats {
                    hp: 0,
                    mp: 0,
                    movement: 1,
                    jump: 1,
                    strength: 0,
                    speed: 10,
                },
                skills: vec![skill::DEBUG_AOE_T_SHAPED],
            },
        );

        result
    }
}

pub const DEBUG_ARMOR_1_ID: ArmorId = 1;
pub const DEBUG_ARMOR_2_ID: ArmorId = 2;
impl ArmorDefinition {
    pub(in crate::game_data) fn mock_armor() -> HashMap<ArmorId, ArmorDefinition> {
        let mut result = HashMap::new();

        result.insert(
            DEBUG_ARMOR_1_ID,
            ArmorDefinition {
                id: DEBUG_ARMOR_1_ID,
                name: "Armor".into(),
                description: "Important Plot Armor!".into(),
                stats: BaseStats {
                    hp: 10,
                    mp: 0,
                    movement: 0,
                    jump: 0,
                    strength: 0,
                    speed: 0,
                },
                skills: vec![],
            },
        );
        result.insert(
            DEBUG_ARMOR_2_ID,
            ArmorDefinition {
                id: DEBUG_ARMOR_2_ID,
                name: "Robes".into(),
                description: "It's magical!".into(),
                stats: BaseStats {
                    hp: 5,
                    mp: 5,
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
                name: "Sword".into(),
                description: "Hit them with the pointy end!".into(),
                stats: BaseStats {
                    hp: 0,
                    mp: 0,
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
                name: "Staff".into(),
                description: "Staff goes bonk!".into(),
                stats: BaseStats {
                    hp: 0,
                    mp: 20,
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
