use crate::in_game::in_game_data::InGameData;
use crate::in_game::states::InGameState;
use crate::message_processor::ServerToClientMessageVariant;
use game_common::player::PlayerId;

pub struct WaitingForOthersTransition {}
pub struct WaitingForOthers {}

impl WaitingForOthersTransition {
    #[must_use]
    pub fn on_state_enter(
        &self,
        in_game_data: &mut InGameData,
        players_in_state: Vec<PlayerId>,
    ) -> Vec<ServerToClientMessageVariant> {
        for player in players_in_state {
            in_game_data
                .insert_state_for_player(player, InGameState::WaitingForOthers(WaitingForOthers {}))
        }

        Vec::new()
    }
}
