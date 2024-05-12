use crate::game::game_plugin::GameState;
use crate::map::map_utils::unit_position_on_hexagon;
use crate::ApplicationState;
use bevy::app::App;
use bevy::math::Vec3;
use bevy::prelude::{
    in_state, Commands, Component, Entity, IntoSystemConfigs, Plugin, Query, Res, Time, Transform,
    Update,
};
use game_common::combat_unit::CombatUnit;
use game_common::game_map::GameMap;
use hexx::Hex;

pub struct UnitAnimationPlugin;
impl Plugin for UnitAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (animate_attack, animate_movement)
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
                transform.translation =
                    unit_position_on_hexagon(move_data.path.last().unwrap().clone(), &map);
                continue;
            }
        }

        let from = unit_position_on_hexagon(move_data.path[move_data.current_step], &map);

        let to = unit_position_on_hexagon(move_data.path[move_data.current_step + 1], &map);

        transform.translation = Vec3::lerp(from, to, move_data.progress);
    }
}

#[derive(Component)]
pub struct UnitAttackAnimationComponent {
    origin: Vec3,
    target: Vec3,
    progress: f32,
}

impl UnitAttackAnimationComponent {
    const ANIMATION_DISTANCE: f32 = 1.0;

    pub fn new(unit: &CombatUnit, target: Hex, map: &GameMap) -> UnitAttackAnimationComponent {
        let origin = unit_position_on_hexagon(unit.position, map);
        if unit.position == target {
            return UnitAttackAnimationComponent {
                origin,
                target: origin + Vec3::Z * Self::ANIMATION_DISTANCE,
                progress: 0.0,
            };
        }

        let target = unit_position_on_hexagon(target, map);

        let direction = Vec3 {
            x: target.x - origin.x,
            y: 0.0,
            z: target.z - origin.z,
        }
        .normalize();

        let target = origin + direction * Self::ANIMATION_DISTANCE;

        UnitAttackAnimationComponent {
            origin,
            target,
            progress: 0.0,
        }
    }
}

pub fn animate_attack(
    mut commands: Commands,
    mut units: Query<(Entity, &mut Transform, &mut UnitAttackAnimationComponent)>,
    time: Res<Time>,
) {
    const ANIMATION_SPEED: f32 = 7.0;

    for (entity, mut transform, mut animation_data) in units.iter_mut() {
        let delta = time.delta_seconds() * ANIMATION_SPEED;
        animation_data.progress += delta;
        if animation_data.progress >= 1.0 {
            commands
                .entity(entity)
                .remove::<UnitAttackAnimationComponent>();
            transform.translation = animation_data.origin;
            continue;
        }

        transform.translation = if animation_data.progress < 0.5 {
            Vec3::lerp(
                animation_data.origin,
                animation_data.target,
                animation_data.progress,
            )
        } else {
            Vec3::lerp(
                animation_data.target,
                animation_data.origin,
                animation_data.progress,
            )
        }
    }
}
