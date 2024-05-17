use crate::in_game::in_game_data::InGameData;
use crate::in_game::states::InGameState;
use crate::message_processor::ServerToClientMessageVariant;
use game_common::player::PlayerId;

pub struct WaitingForOthersTransition {}
pub struct WaitingForOthers {}

impl WaitingForOthersTransition {
    #[must_use]
    pub fn execute(
        &self,
        in_game_data: &mut InGameData,
        affected_players: Vec<PlayerId>,
    ) -> Vec<ServerToClientMessageVariant> {
        let messages = Vec::new();
        for player in affected_players {
            in_game_data.insert_state_for_player(
                player,
                InGameState::WaitingForOthers(WaitingForOthers {}),
            );
        }

        messages
    }
}
