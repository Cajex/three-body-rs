use std::default;
use bevy::app::*;
use bevy::core_pipeline::bloom::{Bloom, BloomCompositeMode};
use bevy::core_pipeline::tonemapping::Tonemapping;
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
    let mut transform = Transform::default();
    transform.scale.x += 65.;
    transform.scale.y += 65.;
    commands.spawn((
        Camera2d,
        Camera {
            hdr: true,
            ..default()
        },
        Tonemapping::TonyMcMapface,
        Bloom {
            intensity: 0.15,
            low_frequency_boost: 0.7,
            low_frequency_boost_curvature: 0.95,
            high_pass_frequency: 1.0,
            composite_mode: BloomCompositeMode::EnergyConserving,
            ..default()
        },
        transform
    ));
}

fn camera_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&Camera, &mut Transform)>,
    time: Res<Time>,
) {
    if let Some((_, mut transform)) = query.iter_mut().next() {
        let mut movement = Vec3::ZERO;
        if keyboard_input.pressed(KeyCode::KeyW) {
            movement.y += 5.0;
        }
        if keyboard_input.pressed(KeyCode::KeyS) {
            movement.y -= 5.0;
        }
        if keyboard_input.pressed(KeyCode::KeyA) {
            movement.x -= 5.0;
        }
        if keyboard_input.pressed(KeyCode::KeyD) {
            movement.x += 5.0;
        }
        transform.translation += movement * time.delta_secs() * 10000.0;
    }
}

fn camera_zoom(
    mut mouse_wheel: EventReader<MouseWheel>,
    mut query: Query<(&Camera, &mut Transform)>,
) {
    mouse_wheel.read().for_each(|mouse_wheel_movement| {
        if let Some((_, mut transform)) = query.iter_mut().next() {
            let zoom_delta = mouse_wheel_movement.y * CAMERA_SPEED;
            transform.scale = transform.scale - Vec3::splat(zoom_delta * 50.);
            transform.scale.x = transform.scale.x.clamp(0.1, 1000.0);
            transform.scale.y = transform.scale.y.clamp(0.1, 1000.0);
        }
    });

}