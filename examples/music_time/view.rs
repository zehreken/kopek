use std::ops::RangeInclusive;

use super::audio::*;
use crate::app::{App, BEAT_COUNT};
use eframe::egui;
use kopek::utils::{self, Keys, C_FREQ};
use ringbuf::{HeapConsumer, HeapProducer, HeapRb};

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[cfg_attr(feature = "persistence", derive(serde::Deserialize, serde::Serialize))]
#[cfg_attr(feature = "persistence", serde(default))] // if we add new fields, give them default values when deserializing old state
pub struct View {
    audio_model: AudioModel,
    input_producer: HeapProducer<Input>,
    view_consumer: HeapConsumer<ViewMessage>,
    beat_views: [Option<ExampleBeatView>; BEAT_COUNT],
    show_modal_window: bool,
    modal_content: ModalContent,
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
        let mut app = App::new(
            audio_model.get_sample_rate(),
            audio_model.get_channel_count(),
            producer,
            input_consumer,
            view_producer,
        )
        .unwrap();
        let _ = std::thread::Builder::new()
            .name("app".to_string())
            .spawn(move || loop {
                app.update();
            });
        let beat_views = [Some(ExampleBeatView::default()); BEAT_COUNT];
        Self {
            audio_model,
            input_producer,
            view_consumer,
            beat_views,
            show_modal_window: false,
            modal_content: ModalContent {
                selected: 0,
                time_signature: (4, 4),
                key: Keys::C,
                bpm: 120,
            },
        }
    }
}

impl eframe::App for View {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let mut beats: [u32; BEAT_COUNT] = [0; BEAT_COUNT];
        while let Some(message) = self.view_consumer.pop() {
            match message {
                ViewMessage::Beat(i, v) => beats[i as usize] = v,
            }
        }
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!(
                "Sample rate: {0}Hz",
                self.audio_model.get_sample_rate()
            ));
            if self.show_modal_window {
                ui.label("time");
                ui.add(
                    egui::DragValue::new(&mut self.modal_content.time_signature.0)
                        .speed(1)
                        .clamp_range(RangeInclusive::new(1, 8)),
                );
                ui.add(
                    egui::DragValue::new(&mut self.modal_content.time_signature.1)
                        .speed(1)
                        .clamp_range(RangeInclusive::new(1, 8)),
                );
                ui.label("bpm");
                ui.add(
                    egui::DragValue::new(&mut self.modal_content.bpm)
                        .speed(10)
                        .clamp_range(RangeInclusive::new(60, 180)),
                );
                ui.label("key");
                egui::ComboBox::from_label("key")
                    .selected_text(format!("{:?}", self.modal_content.key))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut self.modal_content.key, Keys::C, "C");
                        ui.selectable_value(&mut self.modal_content.key, Keys::D, "D");
                        ui.selectable_value(&mut self.modal_content.key, Keys::E, "E");
                        ui.selectable_value(&mut self.modal_content.key, Keys::F, "F");
                        ui.selectable_value(&mut self.modal_content.key, Keys::G, "G");
                        ui.selectable_value(&mut self.modal_content.key, Keys::A, "A");
                        ui.selectable_value(&mut self.modal_content.key, Keys::B, "B");
                    });
                if ui.button("ok").clicked() {
                    let selected = self.modal_content.selected;
                    let time = self.modal_content.time_signature;
                    let key = self.modal_content.key;
                    let bpm = self.modal_content.bpm;
                    self.show_modal_window = false;
                    self.input_producer
                        .push(Input::Create(selected, time, utils::get_freq(key), bpm))
                        .unwrap();
                    self.beat_views[selected] = Some(ExampleBeatView {
                        time_signature: time,
                        key,
                        bpm,
                        is_running: false,
                    });
                }
            } else {
                for (i, beat) in beats.iter().enumerate() {
                    if let Some(mut beat_view) = self.beat_views[i] {
                        let (beat_count, beat_length) = beat_view.time_signature;
                        let bpm = beat_view.bpm;
                        let key = beat_view.key;
                        egui::Frame::none()
                            .fill(egui::Color32::BLACK)
                            .show(ui, |ui| {
                                ui.group(|ui| {
                                    ui.label(format!("{}/{}", beat_count, beat_length));
                                    ui.with_layout(
                                        egui::Layout::left_to_right(egui::Align::TOP),
                                        |ui| {
                                            for i in 0..beat_count as u32 {
                                                if beat % beat_count as u32 == i {
                                                    ui.label("+ ");
                                                } else {
                                                    ui.label("- ");
                                                }
                                            }
                                        },
                                    );
                                    if beat_view.is_running {
                                        if ui.button("■").clicked() {
                                            beat_view.is_running = false;
                                            self.input_producer.push(Input::Toggle(i)).unwrap();
                                        }
                                    } else {
                                        if ui.button("▶").clicked() {
                                            beat_view.is_running = true;
                                            self.input_producer.push(Input::Toggle(i)).unwrap();
                                        }
                                    }
                                    ui.label(format!("bpm: {}\nkey: {:?}", bpm, key));
                                    // You need to write it back to the array
                                    // Since there is no reference but only data
                                    self.beat_views[i] = Some(beat_view);
                                    if ui.button("✖").clicked() {
                                        self.beat_views[i] = None;
                                        self.input_producer.push(Input::Delete(i)).unwrap();
                                    }
                                })
                            });
                    } else {
                        ui.with_layout(egui::Layout::left_to_right(egui::Align::TOP), |ui| {
                            if ui.button("✚").clicked() {
                                self.show_modal_window = true;
                                self.modal_content.selected = i;
                            }
                        });
                    }
                }
            }
        });

        ctx.request_repaint(); // Make UI continuous
        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}

#[derive(Debug)]
pub enum Input {
    Toggle(usize),
    Delete(usize),
    Create(usize, (u8, u8), f32, u16),
}

#[derive(Debug)]
pub enum ViewMessage {
    Beat(u16, u32),
}

#[derive(Clone, Copy)]
struct ExampleBeatView {
    time_signature: (u8, u8),
    key: Keys,
    bpm: u16,
    is_running: bool,
}

#[derive(Clone, Copy)]
struct ModalContent {
    selected: usize,
    time_signature: (u8, u8),
    key: Keys,
    bpm: u16,
}

impl ExampleBeatView {
    fn default() -> Self {
        Self {
            time_signature: (4, 4),
            key: Keys::C,
            bpm: 120,
            is_running: false,
        }
    }
}
