use crate::game::combat::unit_actions::ActiveUnitAction;
use crate::map::{AttackHighlights, CursorOnTile, PathHighlights, RangeHighlights};
use crate::ApplicationState;
use bevy::app::{App, Plugin};
use bevy::prelude::*;
use game_common::combat_data::CombatData;
use game_common::game_data::skill::{SkillId, SkillTargeting};
use game_common::game_data::GameData;
use game_common::game_map::GameMap;
use hexx::Hex;
use std::ops::Deref;

pub struct UnitActionHighlightPlugin;
impl Plugin for UnitActionHighlightPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                on_active_unit_action_changed
                    .run_if(resource_changed_or_removed::<ActiveUnitAction>()),
                (update_path_preview, update_attack_highlights).run_if(
                    resource_changed_or_removed::<ActiveUnitAction>()
                        .or_else(resource_changed_or_removed::<CursorOnTile>()),
                ),
            )
                .run_if(in_state(ApplicationState::InGame)),
        );
    }
}

pub fn on_active_unit_action_changed(
    mut commands: Commands,
    combat_data: Res<CombatData>,
    game_data: Res<GameData>,
    map: Res<GameMap>,
    active_unit_action: Option<Res<ActiveUnitAction>>,
) {
    let Some(active_unit_action) = active_unit_action else {
        commands.remove_resource::<RangeHighlights>();
        return;
    };

    match active_unit_action.deref() {
        ActiveUnitAction::Move => {
            show_movement_range_preview(commands, &combat_data, &map);
        }
        ActiveUnitAction::UseSkill(id) => {
            show_skill_range_preview(id, commands, &combat_data, &map, &game_data);
        }
    }
}

pub fn show_movement_range_preview(
    mut commands: Commands,
    combat_data: &CombatData,
    map: &GameMap,
) {
    let unit = combat_data.current_turn_unit();
    let range = map.field_of_movement(unit, combat_data);

    commands.insert_resource(RangeHighlights { tiles: range })
}

pub fn show_skill_range_preview(
    skill_id: &SkillId,
    mut commands: Commands,
    combat_data: &CombatData,
    map: &GameMap,
    data: &GameData,
) {
    let unit = combat_data.current_turn_unit();
    let skill = &data.skills[skill_id];

    match &skill.targeting {
        SkillTargeting::UserPosition => {
            commands.remove_resource::<RangeHighlights>();
            return;
        }
        SkillTargeting::MouseCursor(range) => {
            let tiles: Vec<Hex> = hexx::algorithms::range_fov(unit.position, range.max, |hex| {
                !map.tiles.contains_key(&hex)
            })
            .into_iter()
            .filter(|x| x.unsigned_distance_to(unit.position) >= range.min)
            .collect();

            commands.insert_resource(RangeHighlights { tiles })
        }
    }
}

pub fn update_path_preview(
    mut commands: Commands,
    combat_data: Res<CombatData>,
    active_unit_action: Option<Res<ActiveUnitAction>>,
    cursor: Option<Res<CursorOnTile>>,
    map: Res<GameMap>,
) {
    let Some(cursor) = cursor else {
        commands.remove_resource::<PathHighlights>();
        return;
    };

    let Some(active_unit_action) = active_unit_action else {
        commands.remove_resource::<PathHighlights>();
        return;
    };

    if active_unit_action.deref() != &ActiveUnitAction::Move {
        commands.remove_resource::<PathHighlights>();
        return;
    }

    let path = map.calculate_path_for_active_unit(&combat_data, cursor.hex);
    if let Some(path) = path {
        commands.insert_resource(PathHighlights { tiles: path })
    } else {
        commands.remove_resource::<PathHighlights>()
    }
}

pub fn update_attack_highlights(
    mut commands: Commands,
    active_unit_action: Option<Res<ActiveUnitAction>>,
    mouse_cursor_on_tile: Option<Res<CursorOnTile>>,
    map: Res<GameMap>,
    combat_data: Res<CombatData>,
    game_data: Res<GameData>,
) {
    let Some(active_unit_action) = active_unit_action else {
        commands.remove_resource::<AttackHighlights>();
        return;
    };

    let Some(mouse_cursor_on_tile) = mouse_cursor_on_tile else {
        commands.remove_resource::<AttackHighlights>();
        return;
    };

    let user_pos = combat_data.current_turn_unit().position;

    match active_unit_action.deref() {
        ActiveUnitAction::Move => commands.remove_resource::<AttackHighlights>(),
        ActiveUnitAction::UseSkill(skill_id) => {
            let skill = &game_data.skills[skill_id];
            commands.insert_resource(AttackHighlights {
                tiles: skill.get_valid_target_hexagons(mouse_cursor_on_tile.hex, user_pos, &map),
            });
        }
    }
}

#[cfg(test)]
mod tests {
    use bevy::app::App;
    use hexx::Hex;

    use crate::ApplicationState;
    use game_common::combat_data::CombatData;
    use game_common::combat_unit::CombatUnit;
    use game_common::game_data::GameData;
    use game_common::game_map::GameMap;

    use crate::game::combat::unit_actions::{ActiveUnitAction, UnitActionPlugin};
    use crate::map::RangeHighlights;
    use crate::networking::NetworkPlugin;

    #[test]
    fn should_create_and_remove_highlights() {
        let mut app = App::new();
        app.add_plugins(UnitActionPlugin);
        app.add_plugins(NetworkPlugin);
        app.insert_state(ApplicationState::InGame);
        app.insert_resource(GameMap::new(1));
        app.insert_resource(GameData::create_mock());

        let unit_id = 1;
        let unit = CombatUnit::create_mock(unit_id, 1).with_position(Hex::ZERO);

        app.insert_resource(
            CombatData::create_mock()
                .with_units(vec![unit])
                .with_unit_turn(unit_id),
        );
        app.insert_resource(ActiveUnitAction::Move);
        app.update();

        assert!(
            app.world.get_resource::<RangeHighlights>().is_some(),
            "HighlightedTiles should have been created!"
        );

        app.world.remove_resource::<ActiveUnitAction>();
        app.update();

        assert!(
            app.world.get_resource::<RangeHighlights>().is_none(),
            "HighlightedTiles should have been removed!"
        );
    }
}
