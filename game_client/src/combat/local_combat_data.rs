use bevy::prelude::{Entity, Resource};
use bevy::utils::HashMap;
use game_common::unit::UnitId;

/// Contains all the things which don't need to be synced via network.
#[derive(Resource)]
pub struct LocalCombatData {
    pub unit_entities: HashMap<UnitId, Entity>,
}
