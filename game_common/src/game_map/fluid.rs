use crate::game_map::fluid_kind::FluidKind;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Fluid {
    pub height: f32,
    pub kind: FluidKind,
}
