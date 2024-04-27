use bevy::prelude::Resource;
use game_common::combat_data::CombatData;
use std::ops::{Deref, DerefMut};

/// Resource Wrapper around CombatData
#[derive(Resource, Debug)]
pub struct CombatDataResource {
    data: CombatData,
}

impl CombatDataResource {
    pub fn new(data: CombatData) -> Self {
        CombatDataResource { data }
    }
}

impl Deref for CombatDataResource {
    type Target = CombatData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for CombatDataResource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
