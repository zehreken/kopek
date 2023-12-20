use super::audio::*;
use crate::app::App;
use eframe::egui;
use kopek::envelope::EnvelopeState;
use ringbuf::{HeapConsumer, HeapProducer, HeapRb};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct View {
    audio_model: AudioModel,
    input_producer: HeapProducer<Input>, // first is freq, second is octave
    view_consumer: HeapConsumer<ViewMessage>,
}

impl Default for View {
    fn default() -> Self {
        let ring = HeapRb::new(2048);
        let (producer, consumer) = ring.split();
        let input_ring = HeapRb::<Input>::new(16);
        let (input_producer, input_consumer) = input_ring.split();
        let view_ring = HeapRb::new(100);
        let (view_producer, view_consumer) = view_ring.split();
        let audio_model = AudioModel::new(consumer).unwrap();
        let mut app = App::new(44100.0, producer, input_consumer, view_producer).unwrap();
        let _ = std::thread::Builder::new()
            .name("app".to_string())
            .spawn(move || loop {
                app.update();
            });
        Self {
            audio_model,
            input_producer,
            view_consumer,
        }
    }
}

impl eframe::App for View {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut beat_4_4 = 0;
        let mut beat_3_4 = 0;
        let mut beat_5_4 = 0;
        let mut envelope_state = EnvelopeState::Idle;
        while let Some(message) = self.view_consumer.pop() {
            match message {
                ViewMessage::Beat4_4(v) => beat_4_4 = v,
                ViewMessage::Beat3_4(v) => beat_3_4 = v,
                ViewMessage::Beat5_4(v) => beat_5_4 = v,
                ViewMessage::State(v) => envelope_state = v,
            }
        }

        check_input(ctx, &mut self.input_producer);

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("Sample rate: {0}Hz", self.audio_model.sample_rate));
            ui.label(format!("Channels: {0}", self.audio_model.channel_count));
            ui.label(format!("Envelope state: {}", envelope_state));
        });

        ctx.request_repaint(); // Make UI continuous
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

fn check_input(ctx: &egui::Context, input_producer: &mut HeapProducer<Input>) {
    ctx.input(|i| {
        i.events.iter().for_each(|e| match e {
            egui::Event::Key {
                key,
                pressed,
                repeat,
                ..
            } => {
                if *key == egui::Key::A {
                    if *pressed && !*repeat {
                        input_producer.push(Input::Pressed).unwrap();
                    } else if !*pressed {
                        input_producer.push(Input::Released).unwrap();
                    }
                }
            }
            _ => {}
        })
    });
}

#[derive(Debug)]
pub enum Input {
    Pressed,
    Released,
}

#[derive(Debug)]
pub enum ViewMessage {
    State(EnvelopeState),
    Beat4_4(u32),
    Beat3_4(u32),
    Beat5_4(u32),
}
