use game_common::combat_data::CombatData;
use game_common::game_map::GameMap;
use game_common::player::PlayerId;
use std::collections::HashMap;

type StateId = u32;

pub struct InGameData {
    last_unused_state_id: StateId,

    /// All states which are currently active
    states: HashMap<StateId, InGameState>,

    /// Maps PlayerIds to their assigned states. Multiple Players might share the same state, and
    /// every player always has a state assigned to them.
    player_states: HashMap<PlayerId, StateId>,
    // TODO: Also Persist which units a player owns and all that other stuff here
}

impl Default for InGameData {
    fn default() -> Self {
        InGameData {
            last_unused_state_id: 0,
            states: Default::default(),
            player_states: Default::default(),
        }
    }
}

impl InGameData {
    pub fn player_state_mut(&mut self, player_id: &PlayerId) -> &mut InGameState {
        self.states
            .get_mut(&self.player_states[&player_id])
            .unwrap()
    }

    pub fn insert_state_for_player(&mut self, player: PlayerId, state: InGameState) {
        let state_id = self.get_unused_state_id();
        self.states.insert(state_id, state);
        self.assign_player_state(player, state_id);
    }

    pub fn add_player_to_other_player_state(&mut self, player: &PlayerId, player_to_add: PlayerId) {
        let state = self.player_states[player];
        self.assign_player_state(player_to_add, state);
    }

    fn get_unused_state_id(&mut self) -> StateId {
        self.last_unused_state_id += 1;
        self.last_unused_state_id
    }

    fn assign_player_state(&mut self, player: PlayerId, state: StateId) {
        if let Some(previous_state) = self.player_states.insert(player, state) {
            if self
                .player_states
                .values()
                .find(|&&x| x == previous_state)
                .is_none()
            {
                self.states.remove(&previous_state);
            }
        }
    }
}

pub enum InGameState {
    Combat(MatchData),
}

pub struct MatchData {
    pub loaded_map: GameMap,
    pub combat_data: CombatData,
}
