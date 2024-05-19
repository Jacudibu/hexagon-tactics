use crate::unit_stats::UnitStats;

#[derive(Copy, Clone)]
pub struct BaseStats {
    pub hp: i32,
    pub mp: i32,
    pub movement: i8,
    pub jump: i8,
    pub strength: i32,
    pub speed: i32,
}

impl std::ops::AddAssign for BaseStats {
    fn add_assign(&mut self, rhs: Self) {
        self.hp += rhs.hp;
        self.mp += rhs.mp;
        self.movement += rhs.movement;
        self.jump += rhs.jump;
        self.strength += rhs.strength;
        self.speed += rhs.speed;
    }
}

impl Into<UnitStats> for BaseStats {
    fn into(self) -> UnitStats {
        UnitStats {
            max_health: Self::clamp_u32(self.hp),
            max_mana: Self::clamp_u32(self.mp),
            movement: Self::clamp_u8(self.movement),
            jump: Self::clamp_u8(self.jump),
            strength: Self::clamp_u32(self.strength),
            speed: Self::clamp_u32(self.speed),
        }
    }
}

impl BaseStats {
    fn clamp_u8(value: i8) -> u8 {
        if value < 0 {
            0
        } else {
            value as u8
        }
    }

    fn clamp_u32(value: i32) -> u32 {
        if value < 0 {
            0
        } else {
            value as u32
        }
    }
}
