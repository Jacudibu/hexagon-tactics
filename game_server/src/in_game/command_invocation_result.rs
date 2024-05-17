use crate::in_game::states::StateTransitionKind;
use crate::message_processor::ServerToClientMessageVariant;
use game_common::player::PlayerId;

#[derive(Default)]
pub struct CommandInvocationResult {
    pub state_transitions: Vec<StateTransition>,
    pub messages: Vec<ServerToClientMessageVariant>,
}

pub struct StateTransition {
    pub players: Vec<PlayerId>,
    pub kind: StateTransitionKind,
}

impl StateTransition {
    pub fn new(player: PlayerId, kind: StateTransitionKind) -> Self {
        Self {
            players: vec![player],
            kind,
        }
    }
}

impl CommandInvocationResult {
    #[must_use]
    pub fn with_state_transition(mut self, state_transition: StateTransition) -> Self {
        self.state_transitions.push(state_transition);
        self
    }

    #[must_use]
    pub fn with_message(mut self, message: ServerToClientMessageVariant) -> Self {
        self.messages.push(message);
        self
    }

    pub fn add_state_transition(&mut self, state_transition: StateTransition) -> &Self {
        self.state_transitions.push(state_transition);
        self
    }

    pub fn add_message(&mut self, message: ServerToClientMessageVariant) -> &Self {
        self.messages.push(message);
        self
    }

    pub fn add_messages(&mut self, messages: &mut Vec<ServerToClientMessageVariant>) -> &Self {
        self.messages.append(messages);
        self
    }
}
