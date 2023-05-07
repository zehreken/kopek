use super::audio::*;
use eframe::egui;
use egui::plot::{Line, Plot, PlotPoints};
use kopek::oscillator::*;
use ringbuf::{HeapConsumer, HeapProducer, HeapRb};
use std::collections::VecDeque;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct View {
    audio_model: AudioModel,
    // input_producer: HeapProducer<Input>, // first is freq, second is octave
    // sample: VecDeque<f32>,
    // view_consumer: HeapConsumer<f32>,
    // octave: u8,
}

impl Default for View {
    fn default() -> Self {
        let ring = HeapRb::new(2048);
        let (producer, consumer) = ring.split();
        // let input_ring = HeapRb::<Input>::new(16);
        // let (input_producer, input_consumer) = input_ring.split();
        // let view_ring = HeapRb::new(100000);
        // let (view_producer, view_consumer) = view_ring.split();
        let audio_model = AudioModel::new(consumer).unwrap();
        std::thread::spawn(move || loop {
            // generator.update();
        });
        Self {
            audio_model,
            // input_producer,
            // sample: VecDeque::from([0.0; 1024]),
            // view_consumer,
            // octave: 1,
        }
    }
}

impl eframe::App for View {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        todo!()
    }
}
