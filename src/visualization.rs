use crate::projection::Projection;
use egui::{Color32, Stroke};
use flowfield_nd::{FlowField, PointCloud};
use ndarray::{Array, Array2, Dimension, IxDyn};
use rand::Rng;
use threegui::{Painter3D, Vec3};

pub fn compute_n_grid(
    proj: &dyn Projection,
    arr: &Array<f32, IxDyn>,
) -> Vec<(Vec3, Vec3)> {
    let mut out = vec![];

    for (idx, _) in arr.indexed_iter() {
        let tl = idx.as_array_view().to_vec();
        let b = proj.project(&tl.iter().map(|p| *p as f32).collect::<Vec<f32>>());

        for dim in 0..arr.ndim() {
            let mut pos = tl.clone();
            pos[dim] += 1;
            //if arr.get(&*pos).is_some() {
                let a = proj.project(&pos.iter().map(|p| *p as f32).collect::<Vec<f32>>());

                out.push((a, b));
            //}
        }
    }

    out
}

pub fn draw_n_grid(
    buf: &[(Vec3, Vec3)],
    paint: &Painter3D,
    stroke: Stroke,
) {
    for &(a, b) in buf {
        paint.line(a, b, stroke);
    }
}

pub fn draw_pcld(
    pcld: &PointCloud,
    proj: &dyn Projection,
    paint: &Painter3D,
    radius: f32,
    color: Color32,
) {
    for pos in pcld.0.outer_iter() {
        let pos3 = proj.project(pos.as_slice().unwrap());
        paint.circle_filled(pos3, radius, color);
    }
}

pub fn random_pcld_uniform(n: usize, volume: &[usize]) -> PointCloud {
    let mut rng = rand::thread_rng();

    // Don't start out of bounds ...
    let margin = 0.5;

    PointCloud(Array2::from_shape_fn((n, volume.len()), |(_, col)| {
        rng.gen_range(margin..=volume[col] as f32 - margin)
    }))
}
