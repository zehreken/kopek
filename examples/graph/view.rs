use super::utils::Point2;
use eframe::{egui, epi};
use egui::{
    plot::{Line, Plot, Values},
    Color32,
};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct AnalysisView {
    player: super::player::Player,
}

impl Default for AnalysisView {
    fn default() -> Self {
        Self {
            player: super::player::Player::new(),
        }
    }
}

impl epi::App for AnalysisView {
    fn name(&self) -> &str {
        "analysis view"
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    /// Put your widgets into a `SidePanel`, `TopPanel`, `CentralPanel`, `Window` or `Area`.
    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        // let Self { label, value } = self;
        self.player.update();
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            egui::menu::bar(ui, |ui| {
                egui::menu::menu(ui, "kopek", |ui| {
                    if ui.button("Quit").clicked() {
                        frame.quit();
                    }
                });

                egui::menu::menu(ui, "Files", |ui| {
                    for (i, path) in super::player::PATHS.iter().enumerate() {
                        if ui.button(path).clicked() {
                            self.player.load_track(path);
                        }
                    }
                });

                ui.separator();

                if ui.button("Play").clicked() {
                    self.player.play();
                    // self.player.record();
                }
                if ui.button("Pause").clicked() {
                    // pause player
                }
                if ui.button("Stop").clicked() {
                    // stop player
                }
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Frequency domain analysis");

            let waveform_line = line_from_points(
                &self.player.get_waveform_graph_points(),
                Color32::from_rgb(200, 100, 100),
            );

            let frequency_line = line_from_points(
                &self.player.get_frequency_graph_points(),
                Color32::from_rgb(100, 200, 100),
            );
            ui.add(Plot::new("graph").line(frequency_line).line(waveform_line));
            egui::warn_if_debug_build(ui);
        });

        ctx.request_repaint(); // Make UI continuous

        // A little sleep to fix curve flickering
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

fn line_from_points(points: &Vec<Point2>, color: Color32) -> Line {
    let mut ys: [f32; 1024] = [0.0; 1024];
    for (i, p) in points.iter().enumerate() {
        if i < 1024 {
            ys[i] = p.y;
        }
    }

    let values = Values::from_ys_f32(&ys);
    let line = Line::new(values).color(color).name("line");

    line
}

impl AnalysisView {
    #[deprecated]
    fn _sin(&self) -> Line {
        Line::new(Values::from_explicit_callback(
            move |x| 0.5 * (2.0 * x).sin() * 1.0,
            ..,
            512,
        ))
        .color(Color32::from_rgb(200, 100, 100))
        .name("wave")
    }
}