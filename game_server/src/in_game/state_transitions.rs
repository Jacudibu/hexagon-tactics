use crate::in_game::in_game_data::InGameData;
use crate::in_game::states::StateTransitionKind;
use crate::message_processor::ServerToClientMessageVariant;
use game_common::player::PlayerId;

#[must_use]
pub fn on_state_enter(
    sender: &PlayerId,
    state_transition: &StateTransitionKind,
    in_game_data: &mut InGameData,
) -> Vec<ServerToClientMessageVariant> {
    let players_in_state = in_game_data.get_all_players_in_same_state(sender);

    match state_transition {
        StateTransitionKind::PickUnit(state) => {
            state.on_state_enter(in_game_data, players_in_state)
        }
        StateTransitionKind::Combat(state) => state.on_state_enter(in_game_data, players_in_state),
        StateTransitionKind::CombatFinished(state) => {
            state.on_state_enter(in_game_data, players_in_state)
        }
    }
}
