use std::f32::consts::PI;
use rand::prelude::*;

use threegui::Vec3;

pub fn project(pos: &[f32], axes: &[Vec3]) -> Vec3 {
    pos.iter().zip(axes).map(|(pos, axis)| *pos * *axis).sum()
}

pub fn generate_axes(dims: usize) -> Vec<Vec3> {
    let mut rng = rand::thread_rng();
    (0..dims).map(|_| gen_rand_vect(&mut rng, dims)).collect()
}

// https://extremelearning.com.au/how-to-evenly-distribute-points-on-a-sphere-more-effectively-than-the-canonical-fibonacci-lattice/
pub fn gen_rand_vect(rng: &mut impl Rng, n: usize) -> Vec3 {
    let golden_ratio = (1.0 + 5_f32.sqrt()) / 2.;
    let i = rng.gen_range(0..=n) as f32;
    let theta = 2. * PI * i / golden_ratio;
    let phi = (1. - 2. * (i + 0.5) / n as f32).acos();
    Vec3::new(theta.cos() * phi.sin(), theta.sin() * phi.sin(), phi.cos())
}
