use crate::combat_unit::UnitId;
use crate::game_data::class::ClassId;
use crate::game_data::equipment::WeaponId;
use crate::game_data::level::Level;
use crate::game_data::race::RaceId;
use crate::game_data::skill::SkillId;
use crate::game_data::{AccessoryId, ArmorId, GameData};
use crate::player::PlayerId;
use crate::unit_stats::UnitStats;
use bevy::prelude::Event;
use bevy::utils::HashMap;
use serde::{Deserialize, Serialize};

#[derive(Event, Serialize, Deserialize, Debug, Clone)]
pub struct UnitDefinition {
    pub id: UnitId,
    pub owner: PlayerId, // TODO: Probably should be an enum - None, Player, NPCEnemy
    pub name: String,
    pub active_class: ClassId,

    pub race: RaceId,
    pub levels: HashMap<ClassId, Level>,
    pub permanently_unlocked_skills: Vec<SkillId>,
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
            result += data.weapons[weapon].stats;
        }
        if let Some(armor) = &self.armor {
            result += data.armor[armor].stats;
        }
        if let Some(accessory) = &self.accessory {
            result += data.accessories[accessory].stats;
        }

        result.into()
    }

    pub fn all_available_skills(&self, data: &GameData) -> Vec<SkillId> {
        let mut result = self.permanently_unlocked_skills.clone();
        if let Some(weapon) = &self.weapon {
            result.extend(&data.weapons[weapon].skills);
        }
        if let Some(armor) = &self.armor {
            result.extend(&data.armor[armor].skills);
        }
        if let Some(accessory) = &self.accessory {
            result.extend(&data.accessories[accessory].skills);
        }

        result
    }
}
