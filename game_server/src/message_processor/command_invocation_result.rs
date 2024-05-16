use crate::message_processor::states::StateTransitionKind;
use crate::message_processor::ServerToClientMessageVariant;

#[derive(Default)]
pub struct CommandInvocationResult {
    pub state_transition: Option<StateTransitionKind>,
    pub messages: Vec<ServerToClientMessageVariant>,
}

impl CommandInvocationResult {
    #[must_use]
    pub fn with_state_transition(mut self, state_transition: StateTransitionKind) -> Self {
        self.state_transition = Some(state_transition);
        self
    }

    #[must_use]
    pub fn with_message(mut self, message: ServerToClientMessageVariant) -> Self {
        self.messages.push(message);
        self
    }

    pub fn set_state_transition(&mut self, state_transition: StateTransitionKind) -> &Self {
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
