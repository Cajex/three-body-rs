use bevy::asset::Assets;
use bevy::color::Color;
use bevy::math::Vec3;
use bevy::prelude::*;
use derive_new::new;
use nalgebra::Vector2;
use uuid::Uuid;

#[derive(Component, Debug, Clone, new)]
pub struct ObjectDescriptor {
    pub m: f64,
    pub id: Uuid,
    pub last_a: f64,
    pub v: Vector2<f64>,
}

pub fn add_object(mut commands: &mut Commands, mut meshes: &mut ResMut<Assets<Mesh>>, mut materials: &mut ResMut<Assets<ColorMaterial>>, m: f64, position: Vector2<f64>, v: Vector2<f64>) {
    commands.spawn((<Handle<Mesh> as Into<Mesh2d>>::into(meshes.add(Circle::new(150.))),
                    <Handle<ColorMaterial> as Into<MeshMaterial2d<ColorMaterial>>>::into(materials.add(ColorMaterial::from_color(Color::BLACK))),
                    Transform::from_translation(Vec3::new(position.x as f32, position.y as f32, 0.)),
                    GlobalTransform::default(),
                    Visibility::default(),
                    InheritedVisibility::default(),
                    ViewVisibility::default(),
                    ObjectDescriptor::new(m, Uuid::new_v4(), 0., v)))
        .with_children(|command| {
            let text_font = TextFont {
                font_size: 80.0,
                ..default()
            };
            command.spawn((
                Text2d::new(format!("m: {} | a: null m/s", m)),
                text_font.clone(),
                Transform::default(),
            ));
    });
}

pub fn update_velocity(mut objects: Query<(&ObjectDescriptor, &Children)>, mut text_child: Query<&mut Text2d>, mut timer: ResMut<CalculateTimer>) {
    if timer.timer.just_finished() {
        objects.iter_mut().for_each(|(desc, mut children)| {
            for &child in children {
                let _text_2d = match text_child.get_mut(child) {
                    Ok(mut text_2d) => {
                        text_2d.0 = format!("m: {0} * m sun | a: {1:.3} km/s²", desc.m, desc.last_a.fract());
                    }
                    Err(_) => {}
                };
            }
        })
    }
}

#[derive(Resource, new)]
pub struct CalculateTimer {
    pub timer: Timer,
}