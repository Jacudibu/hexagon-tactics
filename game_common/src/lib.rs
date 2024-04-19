pub mod combat_data;
pub mod combat_turn;
pub mod game_map;
pub mod network_events;
pub mod player;
pub mod turn_resources;
pub mod unit;
mod unit_stats;

pub const TEST_MAP_NAME: &str = "test_map.map";

/// Using a const makes finding these easier once desyncing is implemented.
pub const DESYNC_TODO_MESSAGE: &str = "TODO: Desync if this ever happens.";
