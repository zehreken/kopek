use super::audio::*;
use eframe::egui;
use egui::Color32;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct View {
    audio_model: Model,
}

impl Default for View {
    fn default() -> Self {
        Self {
            audio_model: Model::new().unwrap(),
        }
    }
}

impl eframe::App for View {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("C").clicked() {}
            if ui.button("D").clicked() {}
            if ui.button("E").clicked() {}
            if ui.button("F").clicked() {}
            if ui.button("G").clicked() {}
            if ui.button("A").clicked() {}
            if ui.button("B").clicked() {}
        });
    }
}
