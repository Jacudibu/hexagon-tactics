use std::fmt::{Display, Formatter};

pub type PlayerId = u8;

#[derive(Debug)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", self.name, self.id)
    }
}
