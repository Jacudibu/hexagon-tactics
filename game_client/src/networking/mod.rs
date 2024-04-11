mod incoming_message_processor;
mod network_plugin;

pub use network_plugin::{
    Network, // TODO: Should probably not be public and instead communicate via events
    NetworkPlugin,
    NetworkState,
};
