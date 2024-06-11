use egui::{Color32, DragValue, Stroke, Vec2};
//use env_logger::fmt::Color;
use flowfield_nd::{sweep_pointcloud, FlowField, FluidSolver, PointCloud, SolverConfig};
use rand::Rng;
use threegui::Vec3;

use crate::{
    projection::{generate_axes, AxisProjection, Projection},
    visualization::{
        compute_n_grid, draw_flowfield_interp_centers, draw_flowfield_staggered, draw_n_grid,
        draw_pcld, random_pcld_uniform,
    },
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
//#[derive(serde::Deserialize, serde::Serialize)]
//#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct DemoApp {
    sim: FluidSolver,
    proj: AxisProjection,

    pcld: PointCloud,
    cfg: SolverConfig,

    grid: Vec<(Vec3, Vec3)>,

    draw_grid: bool,
    draw_centers: bool,
    draw_staggered: bool,
    draw_staggered_dim: usize,
    blower: bool,
    pause: bool,

    last_pcld: Option<Vec<Vec3>>,
}

impl Default for DemoApp {
    fn default() -> Self {
        Self::from_dims(4, 4)
    }
}

impl DemoApp {
    pub fn from_dims(dims: usize, size: usize) -> Self {
        let mut sim = FluidSolver::new(FlowField::new(dims, size));
        let mut rng = rand::thread_rng();

        let s = 1e-1;
        for axis in sim.get_flow_mut().get_axes_mut() {
            for elem in axis {
                *elem = rng.gen_range(-s..=s);
            }
        }

        let proj = AxisProjection::new(sim.dims());

        let grid = compute_n_grid(&proj, &sim.shape());

        let pcld = random_pcld_uniform(2000, &sim.shape());

        let cfg = Default::default();

        Self {
            draw_staggered_dim: 0,
            draw_grid: true,
            draw_centers: true,
            draw_staggered: false,
            blower: true,
            pause: false,
            cfg,
            pcld,
            grid,
            proj,
            sim,
            last_pcld: None,
        }
    }

    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        /*
        if let Some(storage) = cc.storage {
        return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        */

        Default::default()
    }
}

impl eframe::App for DemoApp {
    /*
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
    eframe::set_value(storage, eframe::APP_KEY, self);
    }
    */

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.request_repaint();

        let pcld_3d: Vec<Vec3> = self.pcld.0.outer_iter().map(|pos| self.proj.project(pos.as_slice().unwrap())).collect();

        if !self.pause {
            if self.blower {
                let pos: Vec<usize> = self
                    .sim
                    .get_flow()
                    .shape()
                    .into_iter()
                    .map(|x| x / 2)
                    .collect();
                self.sim.get_flow_mut().get_axes_mut()[0][&*pos] = 1.;
            }

            self.sim.step(&self.cfg);

            sweep_pointcloud(&mut self.pcld, self.sim.get_flow(), self.cfg.dt);
        }

        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            let mut dims = self.sim.dims();
            let mut width = self.sim.width();

            ui.label("Size");
            let resp_dims = ui.add(
                DragValue::new(&mut dims)
                    .prefix("Dimensions: ")
                    .clamp_range(1..=7),
            );
            let resp_width = ui.add(
                DragValue::new(&mut width)
                    .prefix("Width: ")
                    .clamp_range(2..=100),
            );
            let regen = ui.button("Refresh").clicked();
            ui.separator();

            ui.label("Kinetics");
            ui.checkbox(&mut self.blower, "Blower");

            ui.add(
                DragValue::new(&mut self.cfg.dt)
                    .prefix("dt: ")
                    .speed(1e-2)
                    .clamp_range(0.0..=10.0),
            );
            ui.checkbox(&mut self.pause, "Pause");

            ui.separator();

            ui.label("Visualization");
            ui.checkbox(&mut self.draw_grid, "Draw grid");
            ui.horizontal(|ui| {
                ui.checkbox(&mut self.draw_staggered, "Draw storage");
                ui.add(
                    DragValue::new(&mut self.draw_staggered_dim)
                        .prefix("Dimension (0 = all): ")
                        .clamp_range(0..=self.sim.dims()),
                );
            });

            ui.checkbox(&mut self.draw_centers, "Draw centers");

            if resp_dims.changed() || resp_width.changed() || regen {
                *self = Self::from_dims(dims, width);
            }
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                threegui::ThreeWidget::new("widge")
                    .with_desired_size(ui.available_size())
                    .show(ui, |thr| {
                        let paint = thr.painter();
                        /*
                        threegui::utils::grid(
                        &paint,
                        10,
                        1.,
                        Stroke::new(1., Color32::from_gray(40)),
                        );
                        */

                        /*
                        for axis in &self.axes {
                        paint.circle_filled(*axis, 5., Color32::RED);
                        }
                        */

                        if self.draw_grid {
                            draw_n_grid(&self.grid, paint, Stroke::new(1., Color32::from_gray(90)));
                        }

                        if self.draw_centers {
                            draw_flowfield_interp_centers(
                                paint,
                                &self.proj,
                                self.sim.get_flow(),
                                3.,
                            );
                        }

                        if self.draw_staggered {
                            let sel =
                                (self.draw_staggered_dim != 0).then(|| self.draw_staggered_dim - 1);

                            draw_flowfield_staggered(
                                paint,
                                &self.proj,
                                self.sim.get_flow(),
                                3.,
                                sel,
                            );
                        }

                        if let Some(last) = &self.last_pcld {
                            for (a, b) in pcld_3d.iter().zip(last) {
                                paint.line(*a, *b, Stroke::new(1., Color32::WHITE));
                            }
                        }

                        //draw_pcld(&self.pcld, &self.proj, paint, 1., Color32::from_gray(180));
                    })
            });
        });

        if !self.pause {
            self.last_pcld = Some(pcld_3d);
        }
    }
}
