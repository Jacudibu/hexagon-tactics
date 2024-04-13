use crate::combat::combat_input::CombatAction;
use crate::combat::combat_plugin::CombatState;
use crate::map::HighlightedTiles;
use crate::ApplicationState;
use bevy::app::App;
use bevy::prelude::{
    in_state, resource_changed_or_removed, Commands, IntoSystemConfigs, Plugin, Res, Resource,
    Update,
};
use game_common::game_map::GameMap;
use game_common::game_state::CombatData;
use leafwing_input_manager::action_state::ActionState;
use std::ops::Deref;

pub struct UnitActionPlugin;
impl Plugin for UnitActionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                change_action_on_input
                    .run_if(in_state(ApplicationState::InGame))
                    .run_if(in_state(CombatState::ThisPlayerUnitTurn)),
                on_active_unit_action_changed
                    .run_if(resource_changed_or_removed::<ActiveUnitAction>()),
            ),
        );
    }
}

#[derive(Resource, Eq, PartialEq)]
pub enum ActiveUnitAction {
    Move,
}

pub fn change_action_on_input(
    mut commands: Commands,
    action_state: Res<ActionState<CombatAction>>,
    current_action: Option<Res<ActiveUnitAction>>,
) {
    if action_state.just_pressed(&CombatAction::MoveUnit) {
        if let Some(current_action) = current_action {
            if current_action.deref() == &ActiveUnitAction::Move {
                commands.remove_resource::<ActiveUnitAction>();
                return;
            }
        }

        commands.insert_resource(ActiveUnitAction::Move)
    }
}

pub fn on_active_unit_action_changed(
    mut commands: Commands,
    combat_data: Res<CombatData>,
    map: Res<GameMap>,
    active_unit_action: Option<Res<ActiveUnitAction>>,
) {
    let Some(active_unit_action) = active_unit_action else {
        commands.remove_resource::<HighlightedTiles>();
        return;
    };

    match active_unit_action.deref() {
        ActiveUnitAction::Move => {
            show_movement_range_preview(commands, &combat_data, &map);
        }
    }
}

pub fn show_movement_range_preview(
    mut commands: Commands,
    combat_data: &CombatData,
    map: &GameMap,
) {
    let unit = combat_data
        .units
        .get(&combat_data.current_unit_turn.expect("TODO"))
        .expect("TODO");
    let position = unit.position.expect("TODO");

    let range = map.field_of_movement(position, unit);

    commands.insert_resource(HighlightedTiles { tiles: range })
}

#[cfg(test)]
mod tests {
    use crate::combat::unit_actions::{ActiveUnitAction, UnitActionPlugin};
    use crate::map::HighlightedTiles;
    use bevy::app::App;
    use bevy::utils::hashbrown::HashMap;
    use game_common::game_map::GameMap;
    use game_common::game_state::CombatData;
    use game_common::units::Unit;
    use hexx::Hex;

    #[test]
    fn testing_tests_in_bevy() {
        let mut app = App::new();
        app.add_plugins(UnitActionPlugin);
        app.insert_resource(GameMap::new(1));

        let unit_id = 1;
        let mut unit = Unit::create_debug_unit(unit_id, 1, "test".into());
        unit.position = Some(Hex::ZERO);

        let mut units = HashMap::new();
        units.insert(unit_id, unit);
        let mut unit_positions = HashMap::new();
        unit_positions.insert(Hex::ZERO, unit_id);
        let mut turn_order = HashMap::new();
        turn_order.insert(0, unit_id);

        let combat_data = CombatData {
            units,
            unit_positions,
            turn_order,
            units_that_can_still_be_placed: Vec::new(),
            current_unit_turn: Some(unit_id),
        };
        app.insert_resource(combat_data);
        app.insert_resource(ActiveUnitAction::Move);
        app.update();

        assert!(
            app.world.get_resource::<HighlightedTiles>().is_some(),
            "HighlightedTiles should have been created!"
        );

        app.world.remove_resource::<ActiveUnitAction>();
        app.update();

        assert!(
            app.world.get_resource::<HighlightedTiles>().is_none(),
            "HighlightedTiles should have been removed!"
        );
    }
}
