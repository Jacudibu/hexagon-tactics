use crate::game_data::skill::{SkillDefinition, SkillId};
use crate::player::PlayerId;
use crate::unit::UnitId;
use crate::unit_stats::UnitStats;
use base_stats::BaseStats;
use bevy::prelude::{Event, Resource};
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

pub mod base_stats;
pub mod skill;

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

pub const DEBUG_RACE_ID: RaceId = 1;
impl RaceDefinition {
    pub(in crate::game_data) fn mock_data() -> HashMap<RaceId, RaceDefinition> {
        let mut result = HashMap::new();

        result.insert(
            DEBUG_RACE_ID,
            Self {
                id: DEBUG_RACE_ID,
                name: "DEBUG RACE".into(),
                extra_skills: Vec::new(),
                base_stats: BaseStats {
                    movement: 3,
                    jump: 3,
                    strength: 5,
                    speed: 50,
                },
            },
        );

        result
    }
}

pub type ClassId = u32;
pub struct ClassDefinition {
    pub id: ClassId,
    pub name: String,
    pub stats_per_level: BaseStats, // Should we use a constant growth, randomize it a bit (+/- X) or follow a curve?
    pub learnable_skills: Vec<SkillId>,
}

pub type WeaponId = u32;
pub type WeaponDefinition = BaseDefinition<WeaponId>;

pub type ArmorId = u32;
pub type ArmorDefinition = BaseDefinition<ArmorId>;

pub type AccessoryId = u32;
pub type AccessoryDefinition = BaseDefinition<AccessoryId>;

/// Contains hashmaps of all parseable data
#[cfg_attr(feature = "ecs", derive(Resource))]
pub struct GameData {
    pub races: HashMap<RaceId, RaceDefinition>,
    pub classes: HashMap<ClassId, ClassDefinition>,
    pub skills: HashMap<SkillId, SkillDefinition>,
    pub weapons: HashMap<WeaponId, WeaponDefinition>,
    pub armor: HashMap<RaceId, RaceDefinition>,
    pub accessories: HashMap<RaceId, RaceDefinition>,
}

impl GameData {
    pub fn load() -> Self {
        GameData {
            races: RaceDefinition::mock_data(),
            classes: Default::default(),
            skills: SkillDefinition::mock_data(),
            weapons: Default::default(),
            armor: Default::default(),
            accessories: Default::default(),
        }
    }
}

#[cfg(feature = "test_helpers")]
pub mod test_helpers {
    use crate::game_data::skill::SkillDefinition;
    use crate::game_data::GameData;

    impl GameData {
        /// Create empty GameData.
        /// Use `.with_<attribute>` methods to selectively load specific values for tests.
        pub fn create_mock() -> Self {
            GameData {
                races: Default::default(),
                classes: Default::default(),
                skills: Default::default(),
                weapons: Default::default(),
                armor: Default::default(),
                accessories: Default::default(),
            }
        }

        pub fn with_all_mock_skills(mut self) -> Self {
            self.skills = SkillDefinition::mock_data();
            self
        }
    }
}

#[derive(Event, Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Level {
    pub level: u8,
    pub experience: u32,
}

#[derive(Event, Serialize, Deserialize, Debug, Clone)]
pub struct UnitDefinition {
    pub id: UnitId,
    pub owner: PlayerId, // TODO: Probably should be an enum - None, Player, NPCEnemy
    pub name: String,

    pub race: RaceId,
    pub levels: HashMap<ClassId, Level>,
    pub unlocked_skills: Vec<SkillId>,
    pub weapon: Option<WeaponId>,
    pub armor: Option<ArmorId>,
    pub accessory: Option<AccessoryId>,
}

impl PartialEq for UnitDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
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
