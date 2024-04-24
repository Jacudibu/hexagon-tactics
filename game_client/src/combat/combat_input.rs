use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use std::fmt::Formatter;

pub struct CombatInputPlugin;
impl Plugin for CombatInputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<CombatAction>::default());
        app.init_resource::<ActionState<CombatAction>>();
        app.insert_resource(CombatAction::default_input_map());
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Reflect)]
pub enum CombatAction {
    SelectTile,
    NextUnit,
    PreviousUnit,
    MoveUnit,
    Attack,
    EndTurn,
}

impl CombatAction {
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();
        input_map.insert(Self::SelectTile, InputKind::Mouse(MouseButton::Left));
        input_map.insert(Self::NextUnit, KeyCode::KeyF);
        input_map.insert(Self::PreviousUnit, KeyCode::KeyR);
        input_map.insert(Self::MoveUnit, KeyCode::KeyZ);
        input_map.insert(Self::Attack, KeyCode::KeyX);
        input_map.insert(Self::EndTurn, KeyCode::Enter);

        input_map
    }
}
