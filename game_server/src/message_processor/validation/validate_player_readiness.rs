use crate::message_processor::validation::validation_error::ValidationError;
use game_common::player::{Player, PlayerId, ReadyState};
use std::collections::HashMap;

pub fn validate_player_readiness(
    players: &HashMap<PlayerId, Player>,
    required_state: &ReadyState,
) -> Result<(), ValidationError> {
    if players.values().any(|x| &x.ready_state != required_state) {
        match required_state {
            ReadyState::NotReady => Err(ValidationError::new(
                "Not everyone is... not ready yet! Yes, this should probably never happen!",
            )),
            ReadyState::ReadyInLobby => Err(ValidationError::new("Not everyone is ready yet!")),
            ReadyState::LoadedInGame => Err(ValidationError::new(
                "Not everyone has loaded into the game yet!",
            )),
        }
    } else {
        Ok(())
    }
}
