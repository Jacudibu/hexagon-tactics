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

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
enum CombatAction {
    SelectTile,
    NextUnit,
    PreviousUnit,
}

impl std::fmt::Display for CombatAction {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            CombatAction::SelectTile => write!(f, "Select Tile"),
            CombatAction::NextUnit => write!(f, "Next Unit"),
            CombatAction::PreviousUnit => write!(f, "Previous Unit"),
        }
    }
}

impl CombatAction {
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();
        input_map.insert(Self::SelectTile, InputKind::Mouse(MouseButton::Left));
        input_map.insert(Self::NextUnit, KeyCode::KeyF);
        input_map.insert(Self::PreviousUnit, KeyCode::KeyR);

        input_map
    }
}
