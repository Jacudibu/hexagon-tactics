use crate::MouseCursorOverUiState;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;
use leafwing_input_manager::axislike::{DeadZoneShape, DualAxis};
use leafwing_input_manager::buttonlike::MouseWheelDirection;
use leafwing_input_manager::input_map::InputMap;
use leafwing_input_manager::plugin::InputManagerPlugin;
use leafwing_input_manager::prelude::UserInput;
use leafwing_input_manager::user_input::InputKind;
use leafwing_input_manager::{Actionlike, InputManagerBundle};

const MOVEMENT_SPEED: f32 = 20.0;
const ROTATION_SPEED: f32 = 2.5;
const ZOOM_SPEED: f32 = 20.0;
const SUPERSPEED_MULTIPLIER: f32 = 3.0;
const DEFAULT_FOV: f32 = std::f32::consts::FRAC_PI_4; // Same as Projection::Perspective::Default()
const ZOOM_IN_LIMIT: f32 = 0.25;
const ZOOM_OUT_LIMIT: f32 = 1.25;

pub struct CameraPlugin;
impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<CameraAction>::default())
            .add_systems(Startup, spawn_main_camera)
            .add_systems(
                Update,
                zoom_camera.run_if(in_state(MouseCursorOverUiState::NotOverUI)),
            )
            .add_systems(Update, move_camera)
            .add_systems(Update, rotate_camera);
    }
}

#[derive(Component)]
pub struct MainCameraParent {}
#[derive(Component)]
pub struct MainCamera {}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum CameraAction {
    ZoomIn,
    ZoomOut,
    Move,
    Superspeed,
    Up,
    Down,
    Left,
    Right,
    RotateLeft,
    RotateRight,
}

impl CameraAction {
    fn default_input_map() -> InputMap<Self> {
        let mut input_map = InputMap::default();
        input_map.insert(CameraAction::ZoomIn, MouseWheelDirection::Up);
        input_map.insert(CameraAction::ZoomOut, MouseWheelDirection::Down);

        input_map.insert(
            CameraAction::Move,
            UserInput::Single(InputKind::DualAxis(DualAxis::left_stick().with_deadzone(
                DeadZoneShape::Ellipse {
                    radius_x: 0.1,
                    radius_y: 0.1,
                },
            ))),
        );

        input_map.insert(CameraAction::Superspeed, KeyCode::ShiftLeft);

        input_map.insert(CameraAction::Up, KeyCode::ArrowUp);
        input_map.insert(CameraAction::Up, KeyCode::KeyW);
        input_map.insert(CameraAction::Up, GamepadButtonType::DPadUp);

        input_map.insert(CameraAction::Down, KeyCode::ArrowDown);
        input_map.insert(CameraAction::Down, KeyCode::KeyS);
        input_map.insert(CameraAction::Down, GamepadButtonType::DPadDown);

        input_map.insert(CameraAction::Left, KeyCode::ArrowLeft);
        input_map.insert(CameraAction::Left, KeyCode::KeyA);
        input_map.insert(CameraAction::Left, GamepadButtonType::DPadLeft);

        input_map.insert(CameraAction::Right, KeyCode::ArrowRight);
        input_map.insert(CameraAction::Right, KeyCode::KeyD);
        input_map.insert(CameraAction::Right, GamepadButtonType::DPadRight);

        input_map.insert(CameraAction::RotateLeft, KeyCode::KeyQ);
        input_map.insert(CameraAction::RotateRight, KeyCode::KeyE);

        input_map
    }
}

fn spawn_main_camera(mut commands: Commands) {
    let main_camera = commands
        .spawn((
            Name::new("Main Camera"),
            MainCamera {},
            Camera3dBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, 40.0, 40.0),
                    rotation: Quat::from_rotation_x(-std::f32::consts::FRAC_PI_4),
                    ..default()
                },
                ..default()
            },
        ))
        .id();

    commands
        .spawn((
            Name::new("Main Camera Parent"),
            MainCameraParent {},
            FixedCameraRotationData::default(),
            SpatialBundle::default(),
            InputManagerBundle::<CameraAction> {
                input_map: CameraAction::default_input_map(),
                ..default()
            },
        ))
        .add_child(main_camera);
}

fn move_camera(
    time: Res<Time>,
    mut camera: Query<(&mut Transform, &ActionState<CameraAction>), With<MainCameraParent>>,
) {
    if let Ok((mut transform, action_state)) = camera.get_single_mut() {
        let movement = movement_from_input(action_state, &time);
        if let Some(movement) = movement {
            let movement = transform.rotation * movement;
            transform.translation += movement;
        }
    } else {
        error!("Unable to find MainCamera!");
    }
}

#[derive(Component, Default)]
struct FixedCameraRotationData {
    pub target_rotation: u8,
    pub current_rotation: f32,
}

impl FixedCameraRotationData {
    fn remaining_rotation(&self) -> f32 {
        self.target_rotation as f32 - self.current_rotation
    }
}

fn rotate_camera(
    time: Res<Time>,
    mut camera: Query<
        (
            &mut Transform,
            &mut FixedCameraRotationData,
            &ActionState<CameraAction>,
        ),
        With<MainCameraParent>,
    >,
) {
    let Ok((mut transform, mut rotation_data, action_state)) = camera.get_single_mut() else {
        error!("Unable to find MainCamera for fixed camera rotation!");
        return;
    };

    if action_state.just_pressed(&CameraAction::RotateLeft) {
        if rotation_data.target_rotation == 0 {
            rotation_data.target_rotation = 5;
            if rotation_data.current_rotation < 6.0 {
                rotation_data.current_rotation += 6.0;
            }
        } else {
            rotation_data.target_rotation -= 1;
        }
    } else if action_state.just_pressed(&CameraAction::RotateRight) {
        if rotation_data.target_rotation == 5 {
            rotation_data.target_rotation = 0;
            if rotation_data.current_rotation > 0.0 {
                rotation_data.current_rotation -= 6.0;
            }
        } else {
            rotation_data.target_rotation += 1;
        }
    }

    let remaining_rotation = rotation_data.remaining_rotation();
    if remaining_rotation.abs() < 0.0001 {
        return;
    }

    let mut delta = time.delta_seconds() * ROTATION_SPEED;
    if delta > remaining_rotation.abs() {
        delta = remaining_rotation;
    }

    delta = if remaining_rotation < 0.0 {
        delta * -1.0
    } else {
        delta
    };

    rotation_data.current_rotation += delta;
    transform.rotation =
        Quat::from_rotation_y(rotation_data.current_rotation * std::f32::consts::FRAC_PI_3);
}

fn movement_from_input(action_state: &ActionState<CameraAction>, time: &Time) -> Option<Vec3> {
    let mut dir;
    if action_state.pressed(&CameraAction::Move) {
        dir = action_state
            .clamped_axis_pair(&CameraAction::Move)
            .unwrap()
            .xy()
            .extend(0.0);
    } else {
        dir = Vec3::ZERO;
    }

    if action_state.pressed(&CameraAction::Up) {
        dir.z -= 1.0;
    }
    if action_state.pressed(&CameraAction::Down) {
        dir.z += 1.0;
    }
    if action_state.pressed(&CameraAction::Right) {
        dir.x += 1.0;
    }
    if action_state.pressed(&CameraAction::Left) {
        dir.x -= 1.0;
    }

    let speed = {
        if action_state.pressed(&CameraAction::Superspeed) {
            MOVEMENT_SPEED * SUPERSPEED_MULTIPLIER
        } else {
            MOVEMENT_SPEED
        }
    };
    if dir.length() > 1.0 {
        if let Some(dir) = dir.try_normalize() {
            Some(dir * speed * time.delta_seconds())
        } else {
            None
        }
    } else {
        Some(dir * speed * time.delta_seconds())
    }
}

struct CurrentZoom {
    value: f32,
}

impl Default for CurrentZoom {
    fn default() -> Self {
        CurrentZoom { value: 1.0 }
    }
}

fn zoom_camera(
    mut query: Query<(&mut Projection, &Parent), With<MainCamera>>,
    action_state: Query<&ActionState<CameraAction>>,
    time: Res<Time>,
    mut current_zoom: Local<CurrentZoom>,
) {
    let (projection, parent) = query.single_mut();
    let action_state = action_state
        .get(parent.get())
        .expect("Main Camera should always have a parent with action states!");

    if let Some(direction) = zoom_direction(action_state, current_zoom.value) {
        current_zoom.value += direction * ZOOM_SPEED * time.delta_seconds();
        current_zoom.value = current_zoom.value.clamp(ZOOM_IN_LIMIT, ZOOM_OUT_LIMIT);

        match projection.into_inner() {
            Projection::Perspective(projection) => {
                projection.fov = DEFAULT_FOV * current_zoom.value;
            }
            Projection::Orthographic(projection) => {
                projection.scale = current_zoom.value;
            }
        }
    }
}

fn zoom_direction(action_state: &ActionState<CameraAction>, current_scaling: f32) -> Option<f32> {
    if action_state.pressed(&CameraAction::ZoomOut) && current_scaling < ZOOM_OUT_LIMIT {
        Some(1.0)
    } else if action_state.pressed(&CameraAction::ZoomIn) && current_scaling > ZOOM_IN_LIMIT {
        Some(-1.0)
    } else {
        None
    }
}
