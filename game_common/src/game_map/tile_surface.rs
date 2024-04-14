use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum TileSurface {
    Grass,
    Stone,
    Sand,
    Earth,
}

impl Display for TileSurface {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            TileSurface::Grass => write!(f, "Grass"),
            TileSurface::Stone => write!(f, "Stone"),
            TileSurface::Sand => write!(f, "Sand"),
            TileSurface::Earth => write!(f, "Earth"),
        }
    }
}
