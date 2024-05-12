use crate::game_data::skill::{SkillDefinition, SkillId};
use base_stats::BaseStats;
use bevy::prelude::Resource;
use bevy::utils::HashMap;

pub mod base_stats;
pub mod skill;
pub mod unit_definition;

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

pub type MonsterId = u32;
pub struct MonsterDefinition {
    pub id: MonsterId,
    pub name: String,
    pub stats: BaseStats,
    pub skills: Vec<SkillId>,
}
pub const DEBUG_MONSTER_ID: MonsterId = 1;
impl MonsterDefinition {
    pub(in crate::game_data) fn mock_data() -> HashMap<MonsterId, MonsterDefinition> {
        let mut result = HashMap::new();

        result.insert(
            DEBUG_MONSTER_ID,
            Self {
                id: DEBUG_MONSTER_ID,
                name: "DEBUG MONSTER".into(),
                skills: Vec::new(),
                stats: BaseStats {
                    movement: 3,
                    jump: 3,
                    strength: 5,
                    speed: 40,
                },
            },
        );

        result
    }
}

/// Contains hashmaps of all parseable data
#[cfg_attr(feature = "ecs", derive(Resource))]
pub struct GameData {
    pub races: HashMap<RaceId, RaceDefinition>,
    pub classes: HashMap<ClassId, ClassDefinition>,
    pub skills: HashMap<SkillId, SkillDefinition>,
    pub weapons: HashMap<WeaponId, WeaponDefinition>,
    pub armor: HashMap<RaceId, RaceDefinition>,
    pub accessories: HashMap<RaceId, RaceDefinition>,
    pub monsters: HashMap<MonsterId, MonsterDefinition>,
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
            monsters: MonsterDefinition::mock_data(),
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
                monsters: Default::default(),
            }
        }

        pub fn with_all_mock_skills(mut self) -> Self {
            self.skills = SkillDefinition::mock_data();
            self
        }
    }
}
