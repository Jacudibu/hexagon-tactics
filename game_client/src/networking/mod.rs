mod incoming_message_processor;
mod network;
mod network_plugin;

pub use network::Network; // TODO: Should probably not be public and instead communicate via events
pub use network_plugin::{NetworkPlugin, NetworkState};
