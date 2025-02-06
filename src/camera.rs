use bevy::app::*;
use bevy::input::mouse::MouseWheel;
use bevy::input::ButtonInput;
use bevy::math::Vec3;
use bevy::prelude::*;

pub struct CameraPlugin;

pub const CAMERA_SPEED: f32 = 2.;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup).add_systems(Update, (camera_movement, camera_zoom));
    }
}

fn setup(mut commands: Commands) {
    let mut cam_bundle = Camera2dBundle::default();
    cam_bundle.transform.scale.x += 250.;
    cam_bundle.transform.scale.y += 250.;
    commands.spawn(cam_bundle);

}

fn camera_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Camera, &mut Transform)>,
    time: Res<Time>,
) {
    if let Some((_, mut transform)) = query.iter_mut().next() {
        let mut movement = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) {
            movement.y += 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            movement.y -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            movement.x -= 1.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            movement.x += 1.0;
        }
        transform.translation += movement * time.delta_secs() * 100.0;
    }
}

fn camera_zoom(
    mut mouse_wheel: EventReader<MouseWheel>,
    mut query: Query<(&Camera, &mut Transform)>,
) {
    mouse_wheel.read().for_each(|mouse_wheel_movement| {
        if let Some((_, mut transform)) = query.iter_mut().next() {
            let zoom_delta = mouse_wheel_movement.y * CAMERA_SPEED;
            transform.scale = transform.scale - Vec3::splat(zoom_delta * 0.1);
            transform.scale.x = transform.scale.x.clamp(0.1, 10.0);
            transform.scale.y = transform.scale.y.clamp(0.1, 10.0);
        }
    });

}