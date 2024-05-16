use crate::in_game::states::InGameState;
use crate::shared_state::SharedState;
use game_common::player::PlayerId;
use game_common::player_resources::PlayerResources;
use hashbrown::HashMap;

type StateId = u32;

pub struct InGameData {
    last_unused_state_id: StateId,

    /// All states which are currently active
    states: HashMap<StateId, InGameState>,

    /// Maps PlayerIds to their assigned states. Multiple Players might share the same state, and
    /// every player always has a state assigned to them.
    player_states: HashMap<PlayerId, StateId>,

    /// Stores everything a player owns.
    pub player_resources: HashMap<PlayerId, PlayerResources>,
}

impl InGameData {
    pub fn new(shared_state: &SharedState) -> Self {
        let mut data = InGameData {
            last_unused_state_id: 0,
            states: Default::default(),
            player_states: Default::default(),
            player_resources: Default::default(),
        };

        let state = InGameState::StartingGame;
        let state_id = data.get_unused_state_id();

        data.states.insert(state_id, state);
        for (id, _) in &shared_state.players {
            data.assign_player_state(id.clone(), state_id);
            data.player_resources
                .insert(id.clone(), PlayerResources::default());
        }

        data
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

    pub fn get_all_players_in_same_state(&self, player_id: &PlayerId) -> Vec<PlayerId> {
        let player_state = &self.player_states[player_id];
        self.player_states
            .iter()
            .filter_map(|(id, state)| {
                if state == player_state {
                    Some(id.clone())
                } else {
                    None
                }
            })
            .collect()
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

    pub fn deconstruct_for_processing(
        &mut self,
        sender: &PlayerId,
    ) -> (&mut InGameState, &mut HashMap<PlayerId, PlayerResources>) {
        (
            self.states.get_mut(&self.player_states[sender]).unwrap(),
            &mut self.player_resources,
        )
    }
}
