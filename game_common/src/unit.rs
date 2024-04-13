use crate::player::PlayerId;
use crate::unit_stats::UnitStats;
use hexx::Hex;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub type UnitId = u8;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Unit {
    pub id: UnitId,
    pub owner: PlayerId,
    pub name: String,
    pub position: Option<Hex>, // TODO: Once we start differentiating between units in combat and "units stored", this should no longer be an option
    pub hp: u32,
    pub mp: u32,
    pub exp: u32,
    pub base_stats: UnitStats,
    pub stats_after_buffs: UnitStats,
    pub turn_resources: TurnResources,
}

impl PartialEq<Self> for Unit {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", self.name, self.id)
    }
}

impl Unit {
    pub fn create_debug_unit(id: UnitId, owner: PlayerId, name: String) -> Self {
        let movement = 4;

        Unit {
            id,
            owner,
            name,
            position: None,
            hp: 10,
            mp: 10,
            exp: 0,
            base_stats: UnitStats {
                movement,
                jump: 3,
                strength: 10,
            },
            stats_after_buffs: UnitStats {
                movement,
                jump: 3,
                strength: 10,
            },
            turn_resources: TurnResources {
                remaining_movement: movement,
            },
        }
    }
}

#[cfg(feature = "test_helpers")]
pub mod test_helpers {
    use crate::player::PlayerId;
    use crate::unit::{TurnResources, Unit, UnitId};
    use crate::unit_stats::UnitStats;
    use hexx::Hex;

    impl Unit {
        /// Create a mock Unit with sensible defaults.
        /// Use `.with_<attribute>` methods to set specific values for tests.
        pub fn create_mock(id: UnitId, owner: PlayerId) -> Self {
            Unit {
                id,
                owner,
                name: format!("Test Unit #{id}"),
                position: None,
                hp: 10,
                mp: 10,
                exp: 0,
                base_stats: UnitStats::create_mock(),
                stats_after_buffs: UnitStats::create_mock(),
                turn_resources: TurnResources {
                    remaining_movement: UnitStats::create_mock().movement,
                },
            }
        }

        pub fn with_position(mut self, position: Hex) -> Self {
            self.position = Some(position);
            self
        }

        pub fn with_stats(mut self, stats: UnitStats) -> Self {
            self.turn_resources.remaining_movement = stats.movement;
            self.base_stats = stats.clone();
            self.stats_after_buffs = stats;
            self
        }
    }
}

// TODO: These don't need to be unit specific. Persist inside CombatData instead, and refresh when a unit turn starts.
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TurnResources {
    pub remaining_movement: u8,
}
