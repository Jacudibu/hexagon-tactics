use crate::game::combat::unit_placement;
use crate::game::game_plugin::GameState;
use crate::ApplicationState;
use bevy::app::App;
use bevy::math::Vec3;
use bevy::prelude::{
    in_state, Commands, Component, Entity, IntoSystemConfigs, Plugin, Query, Res, Time, Transform,
    Update,
};
use game_common::game_map::GameMap;
use hexx::Hex;

pub struct UnitAnimationPlugin;
impl Plugin for UnitAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            animate_movement
                .run_if(in_state(ApplicationState::InGame))
                .run_if(in_state(GameState::Combat)),
        );
    }
}

#[derive(Component)]
pub struct MoveUnitComponent {
    pub path: Vec<Hex>,
    pub current_step: usize,
    pub progress: f32,
}

impl MoveUnitComponent {
    pub fn new(path: Vec<Hex>) -> Self {
        debug_assert!(
            path.len() > 1,
            "A path needs to consist of at least a start and an end point!"
        );

        MoveUnitComponent {
            path,
            current_step: 0,
            progress: 0.0,
        }
    }
}

pub fn animate_movement(
    mut commands: Commands,
    mut units: Query<(Entity, &mut Transform, &mut MoveUnitComponent)>,
    map: Res<GameMap>,
    time: Res<Time>,
) {
    const UNIT_MOVE_SPEED: f32 = 5.0;

    for (entity, mut transform, mut move_data) in units.iter_mut() {
        let delta = time.delta_seconds() * UNIT_MOVE_SPEED;
        move_data.progress += delta;
        if move_data.progress >= 1.0 {
            if move_data.current_step < move_data.path.len() - 2 {
                move_data.current_step += 1;
                move_data.progress -= 1.0;
                // This is probably where we would pick the appropriate animation to be played
                // (jump or walk)
            } else {
                commands.entity(entity).remove::<MoveUnitComponent>();
                transform.translation = unit_placement::unit_position_on_hexagon(
                    move_data.path.last().unwrap().clone(),
                    &map,
                );
                continue;
            }
        }

        let from =
            unit_placement::unit_position_on_hexagon(move_data.path[move_data.current_step], &map);

        let to = unit_placement::unit_position_on_hexagon(
            move_data.path[move_data.current_step + 1],
            &map,
        );

        transform.translation = Vec3::lerp(from, to, move_data.progress);
    }
}
