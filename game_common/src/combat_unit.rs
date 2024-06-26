use crate::game_data::equipment::WeaponId;
use crate::game_data::equipment::{AccessoryId, ArmorId};
use crate::game_data::monster::{MonsterDefinition, MonsterId};
use crate::game_data::race::{RaceId, DEBUG_RACE_ID};
use crate::game_data::skill::SkillId;
use crate::game_data::unit_definition::UnitDefinition;
use crate::game_data::GameData;
use crate::player::PlayerId;
use crate::unit_stats::UnitStats;
use hexx::Hex;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};
use std::sync::atomic::{AtomicU32, Ordering};

pub type UnitId = u32;
pub fn get_unique_unit_id() -> UnitId {
    static UNIT_ID_COUNTER: AtomicU32 = AtomicU32::new(1);
    UNIT_ID_COUNTER.fetch_add(1, Ordering::Relaxed)
}

/// Contains data that's needed for visuals
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum CombatUnitKind {
    Humanoid(HumanoidData),
    Monster(MonsterData),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct HumanoidData {
    pub race: RaceId,
    pub weapon: Option<WeaponId>,
    pub armor: Option<ArmorId>,
    pub accessory: Option<AccessoryId>,
}

impl HumanoidData {
    pub fn from_unit_definition(unit: &UnitDefinition) -> Self {
        Self {
            race: unit.race,
            weapon: unit.weapon,
            armor: unit.armor,
            accessory: unit.accessory,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MonsterData {
    pub monster_id: MonsterId,
}

#[derive(Debug, Serialize, Deserialize, Copy, Clone, Eq, PartialEq)]
pub enum ActorId {
    Player(PlayerId),
    AI,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CombatUnit {
    pub id: UnitId,
    pub kind: CombatUnitKind,
    pub owner: ActorId,
    pub name: String,
    pub position: Hex,
    pub hp: u32,
    pub mp: u32,
    pub base_stats: UnitStats,
    pub stats_after_buffs: UnitStats,
    pub turn_counter: u32,
    pub turn_tiebreaker: u32,
}

impl PartialEq<Self> for CombatUnit {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Display for CombatUnit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", self.name, self.id)
    }
}

impl From<&MonsterDefinition> for CombatUnit {
    fn from(monster: &MonsterDefinition) -> Self {
        let mut result = Self::create_debug_unit(get_unique_unit_id(), ActorId::AI);
        result.kind = CombatUnitKind::Monster(MonsterData {
            monster_id: monster.id,
        });
        result
    }
}

impl CombatUnit {
    pub fn from_unit_definition(unit: &UnitDefinition, game_data: &GameData) -> Self {
        let kind = CombatUnitKind::Humanoid(HumanoidData::from_unit_definition(unit));
        let stats = unit.calculate_stats(game_data);

        let mut result = CombatUnit {
            id: unit.id,
            owner: ActorId::Player(unit.owner),
            kind,
            name: unit.name.clone(),
            position: Hex::ZERO,
            hp: stats.max_health,
            mp: stats.max_mana,
            base_stats: stats.clone(),
            stats_after_buffs: stats,
            turn_counter: 0,
            turn_tiebreaker: 0,
        };
        result.init_tiebreaker();
        result
    }

    pub fn create_debug_unit(id: UnitId, owner: ActorId) -> Self {
        let movement = 4;
        let max_hp = 10;
        let max_mp = 10;

        let mut result = CombatUnit {
            id,
            owner,
            kind: CombatUnitKind::Humanoid(HumanoidData {
                race: DEBUG_RACE_ID,
                weapon: None,
                accessory: None,
                armor: None,
            }),
            name: format!("Unit {id}"),
            position: Hex::ZERO,
            hp: max_hp,
            mp: max_mp,
            base_stats: UnitStats {
                max_health: max_hp,
                max_mana: max_mp,
                movement,
                jump: 3,
                strength: 10,
                speed: 50,
            },
            stats_after_buffs: UnitStats {
                max_health: max_hp,
                max_mana: max_mp,
                movement,
                jump: 3,
                strength: 10,
                speed: 50,
            },
            turn_counter: 0,
            turn_tiebreaker: 0,
        };
        result.init_tiebreaker();
        result
    }

    pub fn is_dead(&self) -> bool {
        return self.hp == 0;
    }

    fn init_tiebreaker(&mut self) {
        self.turn_tiebreaker = self.base_stats.speed * 1000 + self.id;
    }

    pub fn all_available_skills(&self, game_data: &GameData) -> Vec<SkillId> {
        match &self.kind {
            CombatUnitKind::Humanoid(data) => {
                let mut result = Vec::new();
                if let Some(weapon) = &data.weapon {
                    result.extend(&game_data.weapons[weapon].skills);
                }
                if let Some(armor) = &data.armor {
                    result.extend(&game_data.armor[armor].skills);
                }
                if let Some(accessory) = &data.accessory {
                    result.extend(&game_data.accessories[accessory].skills);
                }

                result
            }
            CombatUnitKind::Monster(_) => Vec::new(),
        }
    }
}

#[cfg(feature = "test_helpers")]
pub mod test_helpers {
    use crate::combat_unit::{ActorId, CombatUnit, CombatUnitKind, HumanoidData, UnitId};
    use crate::game_data::race::DEBUG_RACE_ID;
    use crate::player::PlayerId;
    use crate::unit_stats::UnitStats;
    use hexx::Hex;

    impl CombatUnit {
        /// Create a mock Unit with sensible defaults.
        /// Use `.with_<attribute>` methods to set specific values for tests.
        pub fn create_mock(id: UnitId, owner: PlayerId) -> Self {
            let mut result = CombatUnit {
                id,
                owner: ActorId::Player(owner),
                kind: CombatUnitKind::Humanoid(HumanoidData {
                    race: DEBUG_RACE_ID,
                    weapon: None,
                    accessory: None,
                    armor: None,
                }),
                name: format!("Test Unit #{id}"),
                position: Hex::ZERO,
                hp: 10,
                mp: 10,
                exp: 0,
                base_stats: UnitStats::create_mock(),
                stats_after_buffs: UnitStats::create_mock(),
                turn_counter: 0,
                turn_tiebreaker: 0,
            };

            result.init_tiebreaker();
            result
        }

        pub fn with_position(mut self, position: Hex) -> Self {
            self.position = position;
            self
        }

        pub fn with_stats(mut self, stats: UnitStats) -> Self {
            self.base_stats = stats.clone();
            self.stats_after_buffs = stats;
            self.init_tiebreaker();
            self
        }
    }
}
