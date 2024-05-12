use crate::in_game_state::MatchData;
use crate::message_processor::ServerToClientMessageVariant;
use game_common::combat_turn::CombatTurn;
use game_common::combat_unit::{get_unique_unit_id, CombatUnit, CombatUnitKind, MonsterData};
use game_common::game_data::DEBUG_MONSTER_ID;
use game_common::network_events::server_to_client::{
    PlaceUnit, PlayerTurnToPlaceUnit, ServerToClientMessage, UpdateReadyStateForPlayer,
};
use game_common::player::{Player, PlayerId, ReadyState};
use hexx::Hex;
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

    let mut monster1 = CombatUnit::create_debug_unit(get_unique_unit_id(), 0);
    monster1.position = Hex::new(0, -7);
    monster1.kind = CombatUnitKind::Monster(MonsterData {
        monster_id: DEBUG_MONSTER_ID,
    });
    match_data
        .combat_data
        .unit_positions
        .insert(monster1.position, monster1.id);
    messages.push(ServerToClientMessageVariant::Broadcast(
        ServerToClientMessage::PlaceUnit(PlaceUnit {
            unit: monster1.clone(),
        }),
    ));
    match_data.combat_data.units.insert(monster1.id, monster1);

    let mut monster2 = CombatUnit::create_debug_unit(get_unique_unit_id(), 0);
    monster2.position = Hex::new(3, -7);
    monster2.kind = CombatUnitKind::Monster(MonsterData {
        monster_id: DEBUG_MONSTER_ID,
    });
    match_data
        .combat_data
        .unit_positions
        .insert(monster2.position, monster2.id);
    messages.push(ServerToClientMessageVariant::Broadcast(
        ServerToClientMessage::PlaceUnit(PlaceUnit {
            unit: monster2.clone(),
        }),
    ));
    match_data.combat_data.units.insert(monster2.id, monster2);

    let starting_player_id = players.keys().min().unwrap().clone();
    match_data.combat_data.current_turn = CombatTurn::place_unit(starting_player_id);
    messages.push(ServerToClientMessageVariant::Broadcast(
        ServerToClientMessage::PlayerTurnToPlaceUnit(PlayerTurnToPlaceUnit {
            player: starting_player_id,
        }),
    ));

    Ok(messages)
}
