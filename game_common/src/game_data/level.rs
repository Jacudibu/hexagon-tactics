use bevy::prelude::Event;
use serde::{Deserialize, Serialize};

#[derive(Event, Serialize, Deserialize, Debug, Copy, Clone)]
pub struct Level {
    pub level: u8,
    pub experience: u32,
}

impl Default for Level {
    fn default() -> Self {
        Self {
            level: 1,
            experience: 0,
        }
    }
}

pub enum LevelUp {
    Nothing,
    LevelUp { amount: u8 },
}

impl Level {
    pub fn add_experience(&mut self, amount: u32) -> LevelUp {
        self.experience += amount;

        let mut exp_required = self.total_experience_required_for_next_level();
        if self.experience < exp_required {
            return LevelUp::Nothing;
        }

        let mut amount = 0;
        while self.experience >= exp_required {
            amount += 1;
            self.level += 1;
            self.experience -= exp_required;
            exp_required = self.total_experience_required_for_next_level()
        }

        return LevelUp::LevelUp { amount };
    }

    pub fn total_experience_required_for_next_level(&self) -> u32 {
        100
    }
}
