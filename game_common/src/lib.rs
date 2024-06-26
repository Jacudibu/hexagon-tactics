pub mod combat_data;
pub mod combat_turn;
pub mod combat_unit;
pub mod game_data;
pub mod game_map;
pub mod network_events;
pub mod network_helpers;
pub mod player;
pub mod player_resources;
pub mod turn_resources;
pub mod unit_stats;
pub mod validation;

pub const TEST_MAP_NAME: &str = "test_map.map";

/// Using a const makes finding these easier once desyncing is implemented.
pub const DESYNC_TODO_MESSAGE: &str = "TODO: Desync if this ever happens.";
