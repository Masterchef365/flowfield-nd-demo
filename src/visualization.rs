use crate::projection::Projection;
use egui::{Color32, Stroke};
use flowfield_nd::{combos, fill_shape, FlowField, PointCloud};
use ndarray::{Array, Array2, Dimension, IxDyn};
use rand::Rng;
use threegui::{Painter3D, Vec3};

pub fn compute_n_grid(
    proj: &dyn Projection,
    shape: &[usize],
) -> Vec<(Vec3, Vec3)> {
    let mut out = vec![];

    let shape: Vec<usize> = shape.iter().map(|w| w + 0).collect();

    for tl in fill_shape(&shape) {
        let b = proj.project(&tl.iter().map(|p| *p as f32).collect::<Vec<f32>>());

        for dim in 0..shape.len() {
            let mut pos = tl.clone();
            pos[dim] += 1;

            if pos[dim] < shape[dim] {
            //if arr.get(&*pos).is_some() {

                let a = proj.project(&pos.iter().map(|p| *p as f32).collect::<Vec<f32>>());

                out.push((a, b));
            //}
            }
        }
    }

    out
}

pub fn draw_flowfield_interp_centers(
    paint: &Painter3D,
    proj: &dyn Projection,
    ff: &FlowField,
    scale: f32,
) {
    for cell in combos(0, ff.width() as i32 - 2, 1, ff.dims()) {
        let mut pos: Vec<f32> = cell.iter().map(|c| *c as f32).collect();
        
        pos.iter_mut().for_each(|p| *p += 0.5);
        let interp = ff.n_linear_interp(&pos, flowfield_nd::Boundary::Zero).unwrap();
        let a = proj.project(&pos);

        pos.iter_mut().zip(interp).for_each(|(p, i)| *p += i * scale);
        let b = proj.project(&pos);

        let color = Color32::RED;
        paint.circle_filled(a, 2., color);
        paint.line(a, b, Stroke::new(1., color));
    }
}

pub fn draw_flowfield_staggered(
    paint: &Painter3D,
    proj: &dyn Projection,
    ff: &FlowField,
    scale: f32,
    sel_dim: Option<usize>,
) {
    for (dim, axis) in ff.get_axes().iter().enumerate() {
        if let Some(sel) = sel_dim {
            if dim != sel {
                continue;
            }
        }
        for (idx, value) in axis.indexed_iter() {
            let mut pos: Vec<f32> = idx.as_array_view().iter().map(|c| *c as f32).collect();

            pos.iter_mut().enumerate().for_each(|(idx, p)| if idx != dim { *p -= 0.5 });
            let a = proj.project(&pos);

            pos.iter_mut().enumerate().for_each(|(idx, p)| if idx == dim { *p += scale * *value });
            let b = proj.project(&pos);

            let color = Color32::LIGHT_BLUE;
            paint.circle_filled(a, 2., color);
            paint.line(a, b, Stroke::new(1., color));
        }
    }
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
    //let margin = 0.5;

    PointCloud(Array2::from_shape_fn((n, volume.len()), |(_, col)| {
        rng.gen_range(0.0..volume[col] as f32 - 1.0)
    }))
}
