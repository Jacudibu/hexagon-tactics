use crate::player::PlayerId;
use crate::unit::UnitId;
use crate::unit_stats::UnitStats;
use base_stats::BaseStats;
use bevy::utils::HashMap;

mod base_stats;

// In bevy, all components and resources have to be 'static, so if we don't want to hardcode these,
// we'll need to abstract them a little instead of just holding references to everything.

/// Placeholder Struct holding everything imaginable until we figure out what we really need
pub struct BaseDefinition<T> {
    pub id: T,
    pub name: String,
    pub base_stats: BaseStats,
    pub extra_skills: Vec<SkillId>,
}

pub type RaceId = u32;
pub type RaceDefinition = BaseDefinition<RaceId>;

pub type ClassId = u32;
pub struct ClassDefinition {
    pub id: ClassId,
    pub name: String,
    pub stats_per_level: BaseStats, // Should we use a constant growth, randomize it a bit (+/- X) or follow a curve?
    pub learnable_skills: Vec<SkillId>,
}

pub type SkillId = u32;
pub type SkillDefinition = BaseDefinition<SkillId>;

pub type WeaponId = u32;
pub type WeaponDefinition = BaseDefinition<WeaponId>;

pub type ArmorId = u32;
pub type ArmorDefinition = BaseDefinition<ArmorId>;

pub type AccessoryId = u32;
pub type AccessoryDefinition = BaseDefinition<AccessoryId>;

/// Contains hashmaps of all parseable data
pub struct GameData {
    pub races: HashMap<RaceId, RaceDefinition>,
    pub classes: HashMap<ClassId, ClassDefinition>,
    pub skills: HashMap<SkillId, SkillDefinition>,
    pub weapons: HashMap<WeaponId, WeaponDefinition>,
    pub armor: HashMap<RaceId, RaceDefinition>,
    pub accessories: HashMap<RaceId, RaceDefinition>,
}

pub struct Level {
    pub level: u8,
    pub experience: u32,
}

/// WiP trying to figure out how units would be persisted outside of Combat
pub struct UnitDefinition {
    pub id: UnitId,
    pub owner: PlayerId,
    pub name: String,

    pub race: RaceId,
    pub levels: HashMap<ClassId, Level>,
    pub unlocked_skills: Vec<SkillId>,
    pub weapon: Option<WeaponId>,
    pub armor: Option<ArmorId>,
    pub accessory: Option<AccessoryId>,
}

impl UnitDefinition {
    pub fn calculate_stats(&self, data: &GameData) -> UnitStats {
        let mut result = data.races[&self.race].base_stats.clone();
        for (class_id, level) in &self.levels {
            for _ in 0..level.level {
                result += data.classes[class_id].stats_per_level;
            }
        }
        if let Some(weapon) = &self.weapon {
            result += data.weapons[weapon].base_stats;
        }
        if let Some(armor) = &self.armor {
            result += data.armor[armor].base_stats;
        }
        if let Some(accessory) = &self.accessory {
            result += data.accessories[accessory].base_stats;
        }

        result.into()
    }

    pub fn all_available_skills(&self, data: &GameData) -> Vec<SkillId> {
        let mut result = self.unlocked_skills.clone();
        if let Some(weapon) = &self.weapon {
            result.extend(data.weapons[weapon].extra_skills.clone());
        }
        if let Some(armor) = &self.armor {
            result.extend(data.armor[armor].extra_skills.clone());
        }
        if let Some(accessory) = &self.accessory {
            result.extend(data.accessories[accessory].extra_skills.clone());
        }

        result
    }
}