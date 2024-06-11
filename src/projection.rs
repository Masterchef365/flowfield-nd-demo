use rand::prelude::*;
use std::f32::consts::{PI, TAU};

use threegui::Vec3;

pub fn generate_axes(dims: usize) -> Vec<Vec3> {
    let mut rng = rand::thread_rng();
    if dims <= 3 {
        [Vec3::X, Vec3::Y, Vec3::Z][..dims].to_vec()
    } else if dims == 4 {
        [
            Vec3::new(0., 0., 1.),
            Vec3::new((8.0f32/9.).sqrt(), 0., -1./3.),
            Vec3::new(-(2.0f32/9.).sqrt(), (2.0f32/3.).sqrt(), -1./3.),
            Vec3::new(-(2.0f32/9.).sqrt(), -(2.0f32/3.).sqrt(), -1./3.),
        ].into_iter().map(|c| c / (8.0f32/3.).sqrt()).collect()
    } else {
        (0..dims).map(|_| gen_rand_vect(&mut rng, dims)).collect()
    }
}

// https://extremelearning.com.au/how-to-evenly-distribute-points-on-a-sphere-more-effectively-than-the-canonical-fibonacci-lattice/
pub fn gen_rand_vect(rng: &mut impl Rng, n: usize) -> Vec3 {
    let theta = rng.gen_range(0.0..=PI);
    let phi = rng.gen_range(0.0..=TAU);
    /*
    let n = n * 20;

    let golden_ratio = (1.0 + 5_f32.sqrt()) / 2.;
    let i = rng.gen_range(0..=n) as f32;
    let theta = 2. * PI * i / golden_ratio;
    let phi = (1. - 2. * (i + 0.5) / n as f32).acos();
    */
    Vec3::new(theta.cos() * phi.sin(), theta.sin() * phi.sin(), phi.cos())
}

pub trait Projection {
    fn project(&self, pos: &[f32]) -> Vec3;
}

pub struct AxisProjection {
    axes: Vec<Vec3>,
}

impl AxisProjection {
    pub fn new(dims: usize) -> Self {
        Self {
            axes: generate_axes(dims),
        }
    }

    pub fn dims(&self) -> usize {
        self.axes.len()
    }
}

impl Projection for AxisProjection {
    fn project(&self, pos: &[f32]) -> Vec3 {
        assert_eq!(pos.len(), self.dims());
        pos.iter()
            .zip(&self.axes)
            .map(|(pos, axis)| *pos * *axis)
            .sum()
    }
}
