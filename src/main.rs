mod camera;
mod calculate;
mod api;

use crate::api::*;
use crate::calculate::*;
use crate::camera::CameraPlugin;
use bevy::prelude::*;
use bevy_2d_line::LineRenderingPlugin;
use nalgebra::Vector2;
use std::ops::Add;
use std::time::Duration;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins, CameraPlugin, LineRenderingPlugin))
        .add_systems(Startup, setup)
        .add_systems(Update, (calculate, update_velocity))
        .run();
}

pub fn setup(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.insert_resource(CalculateTimer::new(Timer::new(Duration::from_millis(0), TimerMode::Repeating), 0.));
    add_object(&mut commands, &mut meshes, &mut materials, 1., Vector2::new(0., 0.), Vector2::new(0., 0.));
    add_object(&mut commands, &mut meshes, &mut materials, 0.0003, Vector2::new(-29500., 0.), Vector2::new(0., -54.619954146069));
    add_object(&mut commands, &mut meshes, &mut materials, 0.0003, Vector2::new(29500., 0.), Vector2::new(0., 54.619954146069));


}

pub fn calculate(mut object_query: Query<(&mut Transform, &mut ObjectDescriptor)>, time: Res<Time>, mut timer: ResMut<CalculateTimer>) {
    if timer.timer.tick(time.delta()).just_finished() {
        let objects = object_query.iter().map(|item| { (item.0.clone(), item.1.clone()) }).collect::<Vec<(Transform, ObjectDescriptor)>>();
        object_query.iter_mut().for_each(|(mut transform, mut desc)| {
            let mut sum = vec![];
            objects.iter().for_each(|(object_transform, object_descriptor)| {
                if object_descriptor.id != desc.id {
                    let distance = object_transform.translation - transform.translation;
                    let a = calculate_2b_a(object_descriptor.m, Vector2::new(distance.x as f64, distance.y as f64));


                    sum.push(a);
                }
            });
            calculate_2b_v(sum.iter().copied().sum::<Vector2<f64>>(), timer.current_tick, &mut desc);
            timer.current_tick += time.delta_secs_f64();
        });
        object_query.iter_mut().for_each(|(mut object_transform, mut object_descriptor)| {
            object_transform.translation = object_transform.translation.add(Vec3::new(object_descriptor.v.x as f32, object_descriptor.v.y as f32, 0.));
        })
    }
}

pub fn bounce_back(mut object_query: Query<(&mut Transform, &mut ObjectDescriptor)>) {
    object_query.iter_mut().for_each(|(mut transform, mut object_descriptor)| {
        if transform.translation.x == 5000. || transform.translation.y == 5000. || transform.translation.x == -5000. || transform.translation.y == -5000. {
            object_descriptor.d = object_descriptor.d * -1;
        }
    })
}