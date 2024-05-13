use crate::combat_unit::UnitId;
use crate::game_data::skill::SkillId;
use crate::game_data::{AccessoryId, ArmorId, ClassId, GameData, RaceId, WeaponId};
use crate::player::PlayerId;
use crate::unit_stats::UnitStats;
use bevy::prelude::Event;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

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