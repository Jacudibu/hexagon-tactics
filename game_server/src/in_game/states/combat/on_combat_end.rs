use crate::in_game::command_invocation_result::StateTransition;
use crate::in_game::states::StateTransitionKind;
use crate::message_processor::ServerToClientMessageVariant;
use game_common::combat_data::CombatData;
use game_common::combat_unit::{ActorId, UnitId};
use game_common::game_data::level::LevelUp::LevelUp;
use game_common::game_data::GameData;
use game_common::network_events::server_to_client::{CombatFinished, ServerToClientMessage};
use game_common::player::{Player, PlayerId};
use game_common::player_resources::PlayerResources;
use hashbrown::HashMap;

pub fn on_combat_end(
    players: &mut HashMap<PlayerId, Player>,
    player_resources: &mut HashMap<PlayerId, PlayerResources>,
    combat_data: &CombatData,
    game_data: &GameData,
) -> (StateTransition, Vec<ServerToClientMessageVariant>) {
    let players_in_state: Vec<PlayerId> = players.keys().cloned().collect(); // TODO;
    let winners: Vec<ActorId> = players.keys().map(|&x| ActorId::Player(x)).collect(); // TODO TOO;

    let transition = StateTransition {
        players: players_in_state.clone(),
        kind: StateTransitionKind::CombatFinished(
            crate::in_game::states::combat_finished::CombatFinishedTransition {},
        ),
    };

    let experience = 100;
    let (dead_units, alive_units) = {
        let mut dead_units = Vec::new();
        let mut alive_units = Vec::new();
        for (id, unit) in &combat_data.units {
            if unit.is_dead() {
                dead_units.push(id.clone());
            } else {
                alive_units.push(id.clone());
            }
        }

        (dead_units, alive_units)
    };

    remove_dead_units_from_players(player_resources, &players_in_state, &dead_units);

    for player_id in &players_in_state {
        let resources = player_resources.get_mut(player_id).unwrap();

        for unit in resources
            .units
            .iter_mut()
            .filter(|x| alive_units.contains(&&x.id))
        {
            if let LevelUp { amount } = unit
                .levels
                .get_mut(&unit.active_class)
                .unwrap()
                .add_experience(experience)
            {
                // TODO: Persist skill selection thingy somewhere.
                // Vec<OutstandingSkillSelection> with Vec<SkillId> on Unit?
            }
        }
    }

    let mut messages = Vec::new();
    for player_id in players_in_state {
        messages.push(ServerToClientMessageVariant::SendTo((
            player_id,
            ServerToClientMessage::CombatFinished(CombatFinished {
                winners: winners.clone(),
                casualties: dead_units.clone(),
                experience,
                // TODO: Level Up choices for specific player. In case of multiple level ups for the same unit, only send the first one so they can't peek ahead
            }),
        )));
    }

    (transition, messages)
}

fn remove_dead_units_from_players(
    player_resources: &mut HashMap<PlayerId, PlayerResources>,
    players_in_state: &Vec<PlayerId>,
    dead_units: &Vec<UnitId>,
) {
    for player_id in players_in_state {
        player_resources
            .get_mut(player_id)
            .unwrap()
            .units
            .retain(|x| !dead_units.contains(&x.id))
    }
}
