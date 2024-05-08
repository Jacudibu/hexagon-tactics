use crate::message_processor::ServerToClientMessageVariant;

#[derive(Default)]
pub struct CommandInvocationResult {
    pub state_transition: Option<StateTransition>,
    pub messages: Vec<ServerToClientMessageVariant>,
}

pub enum StateTransition {
    StartCombat,
}

impl CommandInvocationResult {
    #[must_use]
    pub fn with_state_transition(mut self, state_transition: StateTransition) -> Self {
        self.state_transition = Some(state_transition);
        self
    }

    #[must_use]
    pub fn with_message(mut self, message: ServerToClientMessageVariant) -> Self {
        self.messages.push(message);
        self
    }

    pub fn add_state_transition(&mut self, state_transition: StateTransition) -> &Self {
        self.state_transition = Some(state_transition);
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
