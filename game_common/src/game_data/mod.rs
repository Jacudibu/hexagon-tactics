use crate::game_data::equipment::{AccessoryDefinition, AccessoryId, ArmorDefinition, ArmorId};
use crate::game_data::prop::{PropDefinition, PropId};
use crate::game_data::skill::{SkillDefinition, SkillId};
use bevy::prelude::Resource;
use bevy::utils::HashMap;
use class::{ClassDefinition, ClassId};
use equipment::{WeaponDefinition, WeaponId};
use monster::{MonsterDefinition, MonsterId};
use race::{RaceDefinition, RaceId};

pub mod base_stats;
pub mod class;
pub mod equipment;
pub mod level;
pub mod monster;
pub mod prop;
pub mod race;
pub mod skill;
pub mod unit_definition;

/// Contains hashmaps of all parseable data
#[cfg_attr(feature = "ecs", derive(Resource))]
pub struct GameData {
    pub races: HashMap<RaceId, RaceDefinition>,
    pub classes: HashMap<ClassId, ClassDefinition>,
    pub skills: HashMap<SkillId, SkillDefinition>,
    pub weapons: HashMap<WeaponId, WeaponDefinition>,
    pub armor: HashMap<ArmorId, ArmorDefinition>,
    pub accessories: HashMap<AccessoryId, AccessoryDefinition>,
    pub monsters: HashMap<MonsterId, MonsterDefinition>,
    pub props: HashMap<PropId, PropDefinition>,
}

impl GameData {
    pub fn load() -> Self {
        GameData {
            races: RaceDefinition::mock_data(),
            classes: ClassDefinition::mock_data(),
            skills: SkillDefinition::mock_data(),
            weapons: WeaponDefinition::mock_weapons(),
            armor: ArmorDefinition::mock_armor(),
            accessories: AccessoryDefinition::mock_accessories(),
            monsters: MonsterDefinition::mock_data(),
            props: PropDefinition::mock_data(),
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
                props: Default::default(),
            }
        }

        pub fn with_all_mock_skills(mut self) -> Self {
            self.skills = SkillDefinition::mock_data();
            self
        }
    }
}
