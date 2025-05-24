use super::audio::*;
use crate::generator::Generator;
use eframe::egui;
use egui_plot::{Line, Plot, PlotPoints};
use kopek::utils::*;
use ringbuf::{HeapConsumer, HeapProducer, HeapRb};
use std::collections::VecDeque;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct View {
    audio_model: AudioModel,
    input_producer: HeapProducer<Input>, // first is freq, second is octave
    sample: VecDeque<f32>,
    view_consumer: HeapConsumer<f32>,
    octave: u8,
}

impl Default for View {
    fn default() -> Self {
        let ring = HeapRb::new(2048);
        let (producer, consumer) = ring.split();
        let input_ring = HeapRb::<Input>::new(16);
        let (input_producer, input_consumer) = input_ring.split();
        let view_ring = HeapRb::new(100000);
        let (view_producer, view_consumer) = view_ring.split();
        let audio_model = AudioModel::new(consumer).unwrap();
        let sample_rate = audio_model.sample_rate;
        std::thread::spawn(move || {
            let mut generator =
                Generator::new(producer, input_consumer, view_producer, sample_rate).unwrap();
            loop {
                generator.update();
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
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // For visualization
        while let Some(v) = self.view_consumer.pop() {
            self.sample.pop_front();
            self.sample.push_back(v);
        }
        egui::SidePanel::left("left_panel").show(ctx, |ui| {
            ui.label(format!("Sample rate: {0}Hz", self.audio_model.sample_rate));
            ui.add_space(10.0);
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                if ui.button("start").clicked() {
                    self.input_producer.push(Input::Start).unwrap();
                }
                if ui.button("stop").clicked() {
                    self.input_producer.push(Input::Stop).unwrap();
                }
            });
            ui.label("Noise");
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                if ui.button("none").clicked() {
                    self.input_producer.push(Input::ChangeNoise(0)).unwrap();
                }
                if ui.button("random").clicked() {
                    self.input_producer.push(Input::ChangeNoise(1)).unwrap();
                }
                if ui.button("white").clicked() {
                    self.input_producer.push(Input::ChangeNoise(2)).unwrap();
                }
            });
            ui.label("Oscillator");
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                if ui.button("sine").clicked() {
                    self.input_producer
                        .push(Input::ChangeOscillator(0))
                        .unwrap();
                }
                if ui.button("sawtooth").clicked() {
                    self.input_producer
                        .push(Input::ChangeOscillator(1))
                        .unwrap();
                }
                if ui.button("square").clicked() {
                    self.input_producer
                        .push(Input::ChangeOscillator(2))
                        .unwrap();
                }
                if ui.button("triangle").clicked() {
                    self.input_producer
                        .push(Input::ChangeOscillator(3))
                        .unwrap();
                }
                if ui.button("fake sine").clicked() {
                    self.input_producer
                        .push(Input::ChangeOscillator(4))
                        .unwrap();
                }
            });
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                let octave_factor = 2_u8.pow(self.octave as u32) as f32;
                if ui.button("C1").clicked() {
                    self.input_producer
                        .push(Input::ChangeFreq(C_FREQ * octave_factor))
                        .unwrap();
                }
                if ui.button("D").clicked() {
                    self.input_producer
                        .push(Input::ChangeFreq(D_FREQ * octave_factor))
                        .unwrap();
                }
                if ui.button("E").clicked() {
                    self.input_producer
                        .push(Input::ChangeFreq(E_FREQ * octave_factor))
                        .unwrap();
                }
                if ui.button("F").clicked() {
                    self.input_producer
                        .push(Input::ChangeFreq(F_FREQ * octave_factor))
                        .unwrap();
                }
                if ui.button("G").clicked() {
                    self.input_producer
                        .push(Input::ChangeFreq(G_FREQ * octave_factor))
                        .unwrap();
                }
                if ui.button("A").clicked() {
                    self.input_producer
                        .push(Input::ChangeFreq(A_FREQ * octave_factor))
                        .unwrap();
                }
                if ui.button("B").clicked() {
                    self.input_producer
                        .push(Input::ChangeFreq(B_FREQ * octave_factor))
                        .unwrap();
                }
                if ui.button("C2").clicked() {
                    self.input_producer
                        .push(Input::ChangeFreq(C_FREQ * 2.0 * octave_factor))
                        .unwrap();
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
            ui.heading("Time domain");

            self.sample.make_contiguous();
            let waveform_line = Line::new(PlotPoints::from_ys_f32(&self.sample.as_slices().0));

            Plot::new("waveform").show(ui, |plot_ui| plot_ui.line(waveform_line));

            egui::warn_if_debug_build(ui);
        });

        ctx.request_repaint(); // Make UI continuous
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

#[derive(Debug)]
pub enum Input {
    Start,
    Stop,
    ChangeFreq(f32),
    ChangeOscillator(u8),
    ChangeNoise(u8),
}
