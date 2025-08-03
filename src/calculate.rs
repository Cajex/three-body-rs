use bevy::math::NormedVectorSpace;
use nalgebra::Vector2;
use crate::api::ObjectDescriptor;

pub const M_S: f64 = 1.989e30;
pub const AE: f64 = 149597870700.;

pub const G: f64 = 6.67430e-11;

#[test]
pub fn calc() {
    let s = (G * M_S / (29500. / 1000. * AE)).sqrt();
    println!("s: {}", s);
}

pub fn calculate_2b_a(m2: f64, r: Vector2<f64>) -> Vector2<f64> {
    let r = r / 1000. * AE;
    let m2 = m2 * M_S;
    G * m2 / (r.norm() * r.norm()) * r / r.norm() * 1000.
}

/* v-> = vn-1-> + an-> * t */
pub fn calculate_2b_v(a: Vector2<f64>, time: f64, object_descriptor: &mut ObjectDescriptor) {
    let v = object_descriptor.v + a * time * object_descriptor.d as f64;
    object_descriptor.v = v.clone_owned();
}