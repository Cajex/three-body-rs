use bevy::asset::Assets;
use bevy::color::Color;
use bevy::math::Vec3;
use bevy::prelude::*;
use bevy_2d_line::Line;
use derive_new::new;
use nalgebra::Vector2;
use rand::{thread_rng, Rng};
use uuid::Uuid;

#[derive(Component, Clone, new)]
pub struct ObjectDescriptor {
    pub m: f64,
    pub id: Uuid,
    pub last_a: f64,
    pub v: Vector2<f64>,
    pub d: i8,
}

#[derive(Component, Default, new)]
pub struct LineDescriptor {
    id: Uuid
}

#[derive(Resource, new)]
pub struct CalculateTimer {
    pub timer: Timer,
    pub current_tick: f64,
}

pub fn add_object(mut commands: &mut Commands, mut meshes: &mut ResMut<Assets<Mesh>>, mut materials: &mut ResMut<Assets<ColorMaterial>>, m: f64, position: Vector2<f64>, v: Vector2<f64>) {
    let color = Color::srgb_u8(thread_rng().gen_range(0..255), thread_rng().gen_range(0..255), thread_rng().gen_range(0..255));
    let id = Uuid::new_v4();
    commands.spawn((<Handle<Mesh> as Into<Mesh2d>>::into(meshes.add(Circle::new(800.))),
                    <Handle<ColorMaterial> as Into<MeshMaterial2d<ColorMaterial>>>::into(materials.add(ColorMaterial::from_color(color.clone()))),
                    Transform::from_translation(Vec3::new(position.x as f32, position.y as f32, 0.)),
                    GlobalTransform::default(),
                    Visibility::default(),
                    InheritedVisibility::default(),
                    ViewVisibility::default(),
                    ObjectDescriptor::new(m, id, 0., v, 1)))
        .with_children(|command| {
            let text_font = TextFont {
                font_size: 900.0,
                ..default()
            };
            command.spawn((
                Text2d::new(format!("m: {} | a: null m/s", m)),
                text_font.clone(),
                Transform::default(),
            ));
    });



}

pub fn update_velocity(
    mut commands: Commands,
    mut objects: Query<(&ObjectDescriptor, &Transform, &Children)>,
    mut text_child: Query<&mut Text2d>,
    mut timer: ResMut<CalculateTimer>,
    mut lines: Query<(&LineDescriptor, &mut Line)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>
) {
    if timer.timer.just_finished() {
        objects.iter_mut().for_each(|(desc, _, mut children)| {
            for &child in children {
                let _text_2d = match text_child.get_mut(child) {
                    Ok(mut text_2d) => {
                        text_2d.0 = format!("m: {0} * m sun | a: {1:.3} km/s²", desc.m, desc.last_a.fract());
                    }
                    Err(_) => {}
                };
            }
        });
        lines.iter_mut().for_each(|(l_desc, mut line)| {
            objects.iter().for_each(|(desc, transform, children)| {
                if desc.id.eq(&l_desc.id) {
                    line.points.push(Vec2::new(transform.translation.x, transform.translation.y));

                }
            });

        });
    }
}