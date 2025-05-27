use super::audio::*;
use crate::generator::Generator;
use eframe::egui;
use egui::Color32;
use egui_plot::{Line, Plot, PlotPoints};
use kopek::utils::{self};
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
    selected_noise: u8,
    selected_wave: u8,
    selected_freq: u8,
    selected_octave: u8,
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
            selected_noise: 0,
            selected_wave: 0,
            selected_freq: 0,
            selected_octave: 1,
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

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // ui.label(format!("Sample rate: {0}Hz", self.audio_model.sample_rate));
            // ui.add_space(10.0);
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                if ui.button("start").clicked() {
                    self.input_producer.push(Input::Start).unwrap();
                }
                if ui.button("stop").clicked() {
                    self.input_producer.push(Input::Stop).unwrap();
                }
            });
            let cached_noise = self.selected_noise;
            let cached_wave = self.selected_wave;
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.label("noise: ");
                ui.radio_value(&mut self.selected_noise, 0, "none");
                ui.radio_value(&mut self.selected_noise, 1, "random");
                ui.radio_value(&mut self.selected_noise, 2, "white");
                ui.label("oscillator: ");
                ui.radio_value(&mut self.selected_wave, 0, "sine");
                ui.radio_value(&mut self.selected_wave, 4, "fake sine");
                ui.radio_value(&mut self.selected_wave, 3, "triangle");
                ui.radio_value(&mut self.selected_wave, 2, "square");
                ui.radio_value(&mut self.selected_wave, 1, "sawtooth");
            });
            if cached_noise != self.selected_noise {
                self.input_producer
                    .push(Input::ChangeNoise(self.selected_noise))
                    .unwrap();
            }
            if cached_wave != self.selected_wave {
                self.input_producer
                    .push(Input::ChangeOscillator(self.selected_wave))
                    .unwrap();
            }
            let cached_freq = self.selected_freq;
            let cached_octave = self.selected_octave;
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                ui.radio_value(&mut self.selected_freq, 0, "C1");
                ui.radio_value(&mut self.selected_freq, 1, "C#");
                ui.radio_value(&mut self.selected_freq, 2, "D");
                ui.radio_value(&mut self.selected_freq, 3, "D#");
                ui.radio_value(&mut self.selected_freq, 4, "E");
                ui.radio_value(&mut self.selected_freq, 5, "F");
                ui.radio_value(&mut self.selected_freq, 6, "F#");
                ui.radio_value(&mut self.selected_freq, 7, "G");
                ui.radio_value(&mut self.selected_freq, 8, "G#");
                ui.radio_value(&mut self.selected_freq, 9, "A");
                ui.radio_value(&mut self.selected_freq, 10, "A#");
                ui.radio_value(&mut self.selected_freq, 11, "B");
            });
            ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                if ui.button("down").clicked() {
                    if self.selected_octave > 1 {
                        self.selected_octave -= 1;
                    }
                }
                ui.label(format!("octave: {0}", self.selected_octave));
                if ui.button("up").clicked() {
                    if self.selected_octave < 5 {
                        self.selected_octave += 1;
                    }
                }
            });
            if cached_freq != self.selected_freq || cached_octave != self.selected_octave {
                let octave_factor = 2_u8.pow(self.selected_octave as u32) as f32;
                let key = utils::KEYS[self.selected_freq as usize].1;
                let freq = utils::key_to_frequency(key);
                self.input_producer
                    .push(Input::ChangeFreq(freq * octave_factor))
                    .unwrap();
            }
        });
        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            self.sample.make_contiguous();
            let waveform_line = Line::new(PlotPoints::from_ys_f32(&self.sample.as_slices().0));

            let fft_input: Vec<_> = self
                .sample
                .iter()
                .map(|s| std::convert::From::from(*s as f64 / std::i16::MAX as f64))
                .collect();
            let fft_output = kopek::fft::fft(&fft_input);

            let half = fft_output.len() / 8;
            let bin_width = 48_000.0 / fft_output.len() as f64;

            let points: Vec<[f64; 2]> = fft_output[..half]
                .iter()
                .enumerate()
                .map(|(i, complex)| {
                    let freq = i as f64 * bin_width;
                    let magnitude = complex.norm(); // or 20.0 * log10(norm()) if dB scale
                    [freq, magnitude]
                })
                .collect();

            let frequency_line = Line::new(points).color(Color32::GREEN);

            let plot_height = 280.0;

            ui.heading("time domain");
            ui.add_sized([ui.available_width(), plot_height], |ui: &mut egui::Ui| {
                Plot::new("waveform_plot")
                    .show(ui, |plot_ui| {
                        plot_ui.line(waveform_line);
                    })
                    .response
            });

            ui.heading("frequency domain");
            ui.add_sized([ui.available_width(), plot_height], |ui: &mut egui::Ui| {
                Plot::new("frequency_plot")
                    .show(ui, |plot_ui| {
                        plot_ui.line(frequency_line);
                    })
                    .response
            });

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
