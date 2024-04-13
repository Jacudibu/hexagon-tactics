use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct UnitStats {
    pub movement: u8,
    pub jump: u8,
    pub strength: u32,
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
                movement: 3,
                jump: 4,
                strength: 2,
            }
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
