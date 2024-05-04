use bevy::prelude::Resource;
use game_common::game_data::GameData;
use std::ops::{Deref, DerefMut};

/// Resource Wrapper around GameData
#[derive(Resource)]
pub struct GameDataResource {
    data: GameData,
}

impl GameDataResource {
    pub fn new(data: GameData) -> Self {
        GameDataResource { data }
    }
}

impl Deref for GameDataResource {
    type Target = GameData;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

impl DerefMut for GameDataResource {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data
    }
}
