use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnitStats {
    pub max_health: u32,
    pub max_mana: u32,
    pub movement: u8,
    pub jump: u8,
    pub strength: u32,
    pub speed: u32,
    // ...and other stats later on
}

#[cfg(feature = "test_helpers")]
pub mod test_helpers {
    use crate::unit_stats::UnitStats;

    impl UnitStats {
        /// Create mock UnitStats with sensible defaults.
        /// Use `.with_<attribute>` methods to set specific values for tests.
        pub fn create_mock() -> Self {
            UnitStats {
                max_health: 10,
                max_mana: 10,
                movement: 3,
                jump: 4,
                strength: 2,
                speed: 50,
            }
        }

        pub fn with_health(mut self, health: u32) -> Self {
            self.max_health = health;
            self
        }

        pub fn with_movement(mut self, movement: u8) -> Self {
            self.movement = movement;
            self
        }

        pub fn with_jump(mut self, jump: u8) -> Self {
            self.jump = jump;
            self
        }
    }
}
