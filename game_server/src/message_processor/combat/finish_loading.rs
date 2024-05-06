use crate::in_game_state::MatchData;
use crate::message_processor::ServerToClientMessageVariant;
use game_common::combat_turn::CombatTurn;
use game_common::network_events::server_to_client::{
    AddUnitToPlayerStorage, PlayerTurnToPlaceUnit, ServerToClientMessage, UpdateReadyStateForPlayer,
};
use game_common::player::{Player, PlayerId, ReadyState};
use game_common::unit::Unit;
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

    let mut unit_count = 0;
    for (id, _) in players.iter() {
        let unit_a = Unit::create_debug_unit(unit_count + 1, id.clone());
        let unit_b = Unit::create_debug_unit(unit_count + 2, id.clone());
        let unit_c = Unit::create_debug_unit(unit_count + 3, id.clone());
        unit_count += 3;

        match_data.combat_data.unit_storage.push(unit_a.clone());
        match_data.combat_data.unit_storage.push(unit_b.clone());
        match_data.combat_data.unit_storage.push(unit_c.clone());

        messages.push(ServerToClientMessageVariant::Broadcast(
            ServerToClientMessage::AddUnitToPlayerStorage(AddUnitToPlayerStorage { unit: unit_a }),
        ));
        messages.push(ServerToClientMessageVariant::Broadcast(
            ServerToClientMessage::AddUnitToPlayerStorage(AddUnitToPlayerStorage { unit: unit_b }),
        ));
        messages.push(ServerToClientMessageVariant::Broadcast(
            ServerToClientMessage::AddUnitToPlayerStorage(AddUnitToPlayerStorage { unit: unit_c }),
        ));
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
