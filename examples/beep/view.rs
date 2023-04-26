use std::collections::VecDeque;

use crate::generator::Generator;

use super::audio::*;
use eframe::egui;
use egui::plot::{Line, Plot, PlotPoints};
use ringbuf::{HeapConsumer, HeapProducer, HeapRb};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct View {
    audio_model: Model,
    input_producer: HeapProducer<(u8, u8)>, // first is freq, second is octave
    sample: VecDeque<f32>,
    view_consumer: HeapConsumer<f32>,
    octave: u8,
}

impl Default for View {
    fn default() -> Self {
        let ring = HeapRb::new(2048);
        let (producer, consumer) = ring.split();
        let input_ring = HeapRb::<(u8, u8)>::new(16);
        let (input_producer, mut input_consumer) = input_ring.split();
        let view_ring = HeapRb::new(100000);
        let (view_producer, view_consumer) = view_ring.split();
        let audio_model = Model::new(consumer).unwrap();
        let mut generator = Generator::new(producer, input_consumer, view_producer).unwrap();
        std::thread::spawn(move || loop {
            generator.update();
        });
        Self {
            audio_model,
            input_producer,
            sample: VecDeque::from([0.0; 1024]),
            view_consumer,
            octave: 1,
        }
    }
}

impl eframe::App for View {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // For visualization
        while let Some(v) = self.view_consumer.pop() {
            self.sample.pop_front();
            self.sample.push_back(v);
        }
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                if ui.button("start").clicked() {}
                if ui.button("stop").clicked() {}
            });
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                if ui.button("sine").clicked() {
                    self.input_producer.push((8, self.octave)).unwrap();
                }
                if ui.button("sawtooth").clicked() {
                    self.input_producer.push((9, self.octave)).unwrap();
                }
            });
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                if ui.button("C1").clicked() {
                    self.input_producer.push((0, self.octave)).unwrap();
                }
                if ui.button("D").clicked() {
                    self.input_producer.push((1, self.octave)).unwrap();
                }
                if ui.button("E").clicked() {
                    self.input_producer.push((2, self.octave)).unwrap();
                }
                if ui.button("F").clicked() {
                    self.input_producer.push((3, self.octave)).unwrap();
                }
                if ui.button("G").clicked() {
                    self.input_producer.push((4, self.octave)).unwrap();
                }
                if ui.button("A").clicked() {
                    self.input_producer.push((5, self.octave)).unwrap();
                }
                if ui.button("B").clicked() {
                    self.input_producer.push((6, self.octave)).unwrap();
                }
                if ui.button("C2").clicked() {
                    self.input_producer.push((7, self.octave)).unwrap();
                }
            });

            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                if ui.button("down").clicked() {
                    if self.octave > 1 {
                        self.octave -= 1;
                    }
                }
                ui.label(format!("octave: {0}", self.octave));
                if ui.button("up").clicked() {
                    if self.octave < 5 {
                        self.octave += 1;
                    }
                }
            });
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Frequency domain");

            self.sample.make_contiguous();
            let waveform_line = Line::new(PlotPoints::from_ys_f32(&self.sample.as_slices().0));

            Plot::new("waveform").show(ui, |plot_ui| plot_ui.line(waveform_line));

            egui::warn_if_debug_build(ui);
        });

        ctx.request_repaint(); // Make UI continuous
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
