use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

pub type PlayerId = usize;

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Clone)]
pub struct Player {
    pub id: PlayerId,
    pub name: String,
    pub ready_state: ReadyState,
}

#[derive(Debug, Default, Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
pub enum ReadyState {
    #[default]
    NotReady,
    ReadyInLobby,
    LoadedInGame,
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} [{}]", self.name, self.id)
    }
}
