use crate::projection::Projection;
use egui::Stroke;
use ndarray::{Array, Dimension, IxDyn};
use threegui::{Painter3D, Vec3};

pub fn compute_n_grid(
    proj: &dyn Projection,
    arr: &Array<f32, IxDyn>,
) -> Vec<(Vec3, Vec3)> {
    let mut out = vec![];

    for (idx, _) in arr.indexed_iter() {
        let tl = idx.as_array_view().to_vec();
        for dim in 0..arr.ndim() {
            let mut pos = tl.clone();
            pos[dim] += 1;
            if arr.get(&*pos).is_some() {
                let a = proj.project(&pos.iter().map(|p| *p as f32).collect::<Vec<f32>>());
                let b = proj.project(&tl.iter().map(|p| *p as f32).collect::<Vec<f32>>());

                out.push((a, b));
            }
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
