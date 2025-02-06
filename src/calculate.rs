use bevy::math::NormedVectorSpace;
use nalgebra::Vector2;

pub const M_S: f64 = 1.989e30;
pub const AE: f64 = 149597870700.;

pub const G: f64 = 6.67430e-11;

pub fn calculate_2b_a(m2: f64, r: Vector2<f64>) -> Vector2<f64> {
    let r = r * AE / 1000.;
    let m2 = m2 * M_S;
    (- G * m2 / r.norm().sqrt() * r / r.norm()) / AE
}