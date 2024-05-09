use crate::in_game_state::MatchData;
use crate::message_processor::ServerToClientMessageVariant;
use game_common::combat_turn::CombatTurn;
use game_common::network_events::server_to_client::{
    PlayerTurnToPlaceUnit, ServerToClientMessage, UpdateReadyStateForPlayer,
};
use game_common::player::{Player, PlayerId, ReadyState};
use std::collections::HashMap;

pub fn finish_loading(
    sender: PlayerId,
    players: &mut HashMap<PlayerId, Player>,
    match_data: &mut MatchData,
) -> Result<Vec<ServerToClientMessageVariant>, ServerToClientMessage> {
    players.get_mut(&sender).unwrap().ready_state = ReadyState::LoadedInGame;

    let mut messages = vec![ServerToClientMessageVariant::Broadcast(
        ServerToClientMessage::PlayerIsReady(UpdateReadyStateForPlayer {
            player_id: sender,
            ready_state: ReadyState::LoadedInGame,
        }),
    )];

    if players
        .values()
        .any(|x| x.ready_state != ReadyState::LoadedInGame)
    {
        return Ok(messages);
    }

    let starting_player_id = players.keys().min().unwrap().clone();
    match_data.combat_data.current_turn = CombatTurn::place_unit(starting_player_id);
    messages.push(ServerToClientMessageVariant::Broadcast(
        ServerToClientMessage::PlayerTurnToPlaceUnit(PlayerTurnToPlaceUnit {
            player: starting_player_id,
        }),
    ));

    Ok(messages)
}
