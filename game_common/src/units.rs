use std::fmt::{Display, Formatter};

type PlayerId = u8;
type UnitId = u8;

pub struct Unit {
    pub id: UnitId,
    pub owner: PlayerId,
    pub name: String,
    pub hp: u32,
    pub mp: u32,
    pub exp: u32,
    pub base_stats: UnitStats,
    pub stats_after_buffs: UnitStats,
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", self.name, self.id)
    }
}

pub struct Player {
    pub id: PlayerId,
    pub name: String,
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", self.name, self.id)
    }
}

pub struct UnitStats {
    pub movement: u8,
    pub jump: u8,
    pub strength: u32,
    // ...and other stats later on
}
