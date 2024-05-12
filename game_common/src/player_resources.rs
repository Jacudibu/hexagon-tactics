use crate::game_data::unit_definition::UnitDefinition;
use bevy::prelude::Resource;

/// General Resources available to player, shared between client and server.
/// Things in here should always be kept in sync.
#[derive(Default)]
#[cfg_attr(feature = "ecs", derive(Resource))]
pub struct PlayerResources {
    pub units: Vec<UnitDefinition>,
}
