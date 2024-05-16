pub mod combat;
pub mod combat_finished;
pub mod pick_unit;

use crate::message_processor::states::combat::{CombatState, CombatStateTransition};
use crate::message_processor::states::combat_finished::{
    CombatFinishedState, CombatFinishedTransition,
};
use crate::message_processor::states::pick_unit::{PickUnitState, PickUnitStateTransition};

pub enum StateTransitionKind {
    Combat(CombatStateTransition),
    CombatFinished(CombatFinishedTransition),
    PickUnit(PickUnitStateTransition),
}

pub enum InGameState {
    StartingGame,
    PickUnit(PickUnitState),
    Combat(CombatState),
    CombatFinished(CombatFinishedState),
}
