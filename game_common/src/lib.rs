pub mod combat_data;
pub mod game_map;
pub mod network_events;
mod player;
pub mod turn_resources;
pub mod unit;
mod unit_stats;
mod validation_error;

pub const TEST_MAP_NAME: &str = "test_map.map";

/// Using a const makes finding these easier once desyncing is implemented.
pub const DESYNC_TODO_MESSAGE: &str = "TODO: Desync if this ever happens.";
