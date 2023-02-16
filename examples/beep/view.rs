use super::audio::*;
use eframe::egui;
use egui::Color32;
use kopek::wave::*;
use rand::prelude::*;
use ringbuf::{HeapProducer, HeapRb};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct View {
    audio_model: Model,
    input_producer: HeapProducer<u8>,
}

impl Default for View {
    fn default() -> Self {
        let ring = HeapRb::new(2048);
        let (mut producer, consumer) = ring.split();
        let input_ring = HeapRb::new(16);
        let (input_producer, mut input_consumer) = input_ring.split();
        let audio_model = Model::new(consumer).unwrap();
        std::thread::spawn(move || {
            let mut tick = 0.0;
            let mut freq = A_FREQ;
            loop {
                for _ in 0..1024 {
                    if !producer.is_full() {
                        // producer.push(kopek::wave::sine(freq, tick)).unwrap();
                        // producer.push(kopek::wave::white_noise()).unwrap();
                        // println!("value: {}", kopek::wave::saw(freq, tick));
                        producer.push(kopek::wave::saw(freq, tick)).unwrap();
                        tick += 1.0;
                    }
                }
                // std::thread::sleep(std::time::Duration::from_millis(10));
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
                    if input == 7 {}
                }
            }
        });
        Self {
            audio_model,
            input_producer,
        }
    }
}

impl eframe::App for View {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            if ui.button("C").clicked() {
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
        });
    }
}
