pub mod combat;
pub mod combat_finished;
pub mod pick_unit;

use crate::in_game::states::combat::{CombatState, CombatStateTransition};
use crate::in_game::states::combat_finished::{CombatFinishedState, CombatFinishedTransition};
use crate::in_game::states::pick_unit::{PickUnitState, PickUnitStateTransition};

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
