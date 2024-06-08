use egui::{Color32, Stroke, Vec2};
use env_logger::fmt::Color;
use threegui::Vec3;

use crate::projection::generate_axes;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct DemoApp {
    #[serde(skip)] // This how you opt-out of serialization of a field
    axes: Vec<Vec3>,

    dims: usize,
}

impl Default for DemoApp {
    fn default() -> Self {
        let dims = 4;
        Self {
            dims,
            axes: generate_axes(dims),
        }
    }
}

impl DemoApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }

        Default::default()
    }
}

impl eframe::App for DemoApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
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
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                threegui::ThreeWidget::new("widge")
                    .with_desired_size(ui.available_size())
                    .show(ui, |thr| {
                        let paint = thr.painter();
                        threegui::utils::grid(
                            &paint,
                            10,
                            1.,
                            Stroke::new(1., Color32::from_gray(40)),
                        );
                        for axis in &self.axes {
                            paint.circle_filled(*axis, 5., Color32::RED);
                        }
                    })
            });
        });
    }
}
