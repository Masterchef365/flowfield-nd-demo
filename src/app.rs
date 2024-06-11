use egui::{Color32, DragValue, Stroke, Vec2};
use env_logger::fmt::Color;
use flowfield_nd::{sweep_pointcloud, FlowField, FluidSolver, PointCloud, SolverConfig};
use rand::Rng;
use threegui::Vec3;

use crate::{projection::{generate_axes, AxisProjection}, visualization::{compute_n_grid, draw_flowfield_interp_centers, draw_flowfield_staggered, draw_n_grid, draw_pcld, random_pcld_uniform}};

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
}

impl Default for DemoApp {
    fn default() -> Self {
        Self::from_dims(3, 5)
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

        let pcld = random_pcld_uniform(1000, &sim.shape());

        let cfg = Default::default();

        Self {
            draw_staggered_dim: 0,
            draw_grid: true,
            draw_centers: true,
            draw_staggered: true,
            cfg,
            pcld,
            grid,
            proj,
            sim,
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

        self.sim.step(&self.cfg);

        sweep_pointcloud(&mut self.pcld, self.sim.get_flow(), self.cfg.dt);

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);

                let mut dims = self.sim.dims();
                let mut width = self.sim.width();
                let resp_dims = ui.add(DragValue::new(&mut dims).prefix("dims: ").clamp_range(1..=24));
                let resp_width = ui.add(DragValue::new(&mut width).prefix("width: ").clamp_range(2..=100));
                let regen = ui.button("Refresh").clicked();

                ui.checkbox(&mut self.draw_grid, "Draw grid");
                ui.checkbox(&mut self.draw_staggered, "Draw storage");
                ui.add(DragValue::new(&mut self.draw_staggered_dim).clamp_range(0..=self.sim.dims()));
                ui.checkbox(&mut self.draw_centers, "Draw centers");

                ui.add(DragValue::new(&mut self.cfg.dt).prefix("dt: ").speed(1e-2).clamp_range(0.0..=10.0));

                if resp_dims.changed() || resp_width.changed() || regen {
                    *self = Self::from_dims(dims, width);
                }
            });
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
                            draw_flowfield_interp_centers(paint, &self.proj, self.sim.get_flow(), 3.);
                        }

                        if self.draw_staggered {
                            let sel = (self.draw_staggered_dim != 0).then(|| self.draw_staggered_dim - 1);

                            draw_flowfield_staggered(paint, &self.proj, self.sim.get_flow(), 3., sel);
                        }

                        draw_pcld(&self.pcld, &self.proj, paint, 1., Color32::from_gray(180));
                    })
            });
        });
    }
}
