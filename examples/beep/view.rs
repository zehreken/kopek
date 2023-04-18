use std::collections::VecDeque;

use super::audio::*;
use eframe::egui;
use egui::plot::{Line, Plot, PlotPoints};
use kopek::wave::*;
use ringbuf::{HeapConsumer, HeapProducer, HeapRb};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct View {
    audio_model: Model,
    input_producer: HeapProducer<u8>,
    sample: VecDeque<f32>,
    view_consumer: HeapConsumer<f32>,
    octave: i8,
}

impl Default for View {
    fn default() -> Self {
        let ring = HeapRb::new(2048);
        let (mut producer, consumer) = ring.split();
        let input_ring = HeapRb::new(16);
        let (input_producer, mut input_consumer) = input_ring.split();
        let view_ring = HeapRb::new(100000);
        let (mut view_producer, view_consumer) = view_ring.split();
        let audio_model = Model::new(consumer).unwrap();
        std::thread::spawn(move || {
            let mut tick = 0.0;
            let mut freq = A_FREQ;
            loop {
                for _ in 0..1024 {
                    if !producer.is_full() {
                        // let value = kopek::wave::saw(freq, tick);
                        let value = kopek::wave::sine(freq, tick);
                        // let value = kopek::wave::white_noise();
                        // let value = kopek::wave::rand_noise();
                        producer.push(value).unwrap();
                        if !view_producer.is_full() {
                            view_producer.push(value).unwrap();
                        }
                        tick += 1.0;
                    }
                }
                if let Some(input) = input_consumer.pop() {
                    if input == 0 {
                        freq = C_FREQ;
                    }
                    if input == 1 {
                        freq = D_FREQ;
                    }
                    if input == 2 {
                        freq = E_FREQ;
                    }
                    if input == 3 {
                        freq = F_FREQ;
                    }
                    if input == 4 {
                        freq = G_FREQ;
                    }
                    if input == 5 {
                        freq = A_FREQ;
                    }
                    if input == 6 {
                        freq = B_FREQ;
                    }
                    if input == 7 {
                        freq = C_FREQ * 2.0;
                    }
                }
            }
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
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // For visualization
        while let Some(v) = self.view_consumer.pop() {
            self.sample.pop_front();
            self.sample.push_back(v);
        }
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            if ui.button("C1").clicked() {
                self.input_producer.push(0).unwrap();
            }
            if ui.button("D").clicked() {
                self.input_producer.push(1).unwrap();
            }
            if ui.button("E").clicked() {
                self.input_producer.push(2).unwrap();
            }
            if ui.button("F").clicked() {
                self.input_producer.push(3).unwrap();
            }
            if ui.button("G").clicked() {
                self.input_producer.push(4).unwrap();
            }
            if ui.button("A").clicked() {
                self.input_producer.push(5).unwrap();
            }
            if ui.button("B").clicked() {
                self.input_producer.push(6).unwrap();
            }
            if ui.button("C2").clicked() {
                self.input_producer.push(7).unwrap();
            }
            if ui.button("down").clicked() {
                if self.octave > 1 {
                    self.octave /= 2;
                }
            }
            ui.label(format!("octave: {0}", self.octave));
            if ui.button("up").clicked() {
                if self.octave < 16 {
                    self.octave *= 2;
                }
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            ui.heading("Frequency domain analysis");

            self.sample.make_contiguous();
            let waveform_line = Line::new(PlotPoints::from_ys_f32(&self.sample.as_slices().0));

            Plot::new("waveform").show(ui, |plot_ui| plot_ui.line(waveform_line));

            egui::warn_if_debug_build(ui);
        });

        ctx.request_repaint(); // Make UI continuous
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
