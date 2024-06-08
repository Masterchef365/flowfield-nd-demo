use ndarray::{Array, IxDyn};
use threegui::Painter3D;
use crate::projection::Projection;

pub fn draw_n_grid(paint: &Painter3D, proj: &dyn Projection, arr: &Array<f32, IxDyn>) {
    for (idx, _) in arr.indexed_iter() {
        dbg!(idx);
    }
}
