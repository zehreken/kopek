use kopek::{oscillator::*, time_signature::TimeSignature, utils::C_FREQ};
use ringbuf::{HeapConsumer, HeapProducer};

use crate::view::{Input, ViewMessage};
pub const BEAT_COUNT: usize = 4; // Number of time_signatures(beat)

pub struct App {
    sample_rate: f32,
    channel_count: u16,
    tick: u32,
    oscillator: Oscillator,
    producer: HeapProducer<f32>,
    input_consumer: HeapConsumer<Input>,
    view_producer: HeapProducer<ViewMessage>,
    beats: [Option<ExampleBeat>; BEAT_COUNT],
}

impl App {
    pub fn new(
        sample_rate: f32,
        channel_count: u16,
        producer: HeapProducer<f32>,
        input_consumer: HeapConsumer<Input>,
        view_producer: HeapProducer<ViewMessage>,
    ) -> Result<App, anyhow::Error> {
        let example_beat = ExampleBeat::new((4, 4), 120, C_FREQ, sample_rate as u32, channel_count);
        Ok(App {
            sample_rate,
            channel_count,
            tick: 0,
            oscillator: Oscillator::new(sample_rate),
            producer,
            input_consumer,
            view_producer,
            beats: [Some(example_beat); BEAT_COUNT],
        })
    }

    pub fn update(&mut self) {
        for _ in 0..1024 {
            if !self.producer.is_full() {
                let mut value = 0.0;
                for i in 0..BEAT_COUNT {
                    if let Some(mut beat) = self.beats[i] {
                        let (show, accent) = beat.time_signature.update();
                        if show && beat.is_running {
                            let accent_multiplier = if accent { 2.0 } else { 1.0 };
                            value += self
                                .oscillator
                                .sine(beat.key * 16.0 * accent_multiplier, self.tick as f32);
                        }
                        self.beats[i] = Some(beat);
                    }
                }

                self.producer.push(value).unwrap();
                self.tick += 1;
            }
        }

        while let Some(message) = self.input_consumer.pop() {
            match message {
                Input::Toggle(i) => {
                    if let Some(mut beat) = self.beats[i] {
                        beat.toggle();
                        self.beats[i] = Some(beat);
                    }
                }
                Input::Delete(i) => self.beats[i] = None,
                Input::Create(i, time, key, bpm) => {
                    let mut new_beat = ExampleBeat::new(
                        time,
                        bpm,
                        key,
                        self.sample_rate as u32,
                        self.channel_count,
                    );
                    new_beat.sync(self.tick);
                    self.beats[i] = Some(new_beat);
                }
            }
        }

        if self.view_producer.free_len() >= 5 {
            let mut beat_index = 0;
            for beat in self.beats {
                if let Some(beat) = beat {
                    self.view_producer
                        .push(ViewMessage::Beat(
                            beat_index,
                            beat.time_signature.get_beat_index(),
                        ))
                        .unwrap();
                    beat_index += 1;
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct ExampleBeat {
    time_signature: TimeSignature,
    key: f32,
    oscillation: u8,
    is_running: bool,
}

impl ExampleBeat {
    pub fn new(time: (u8, u8), bpm: u16, key: f32, sample_rate: u32, channel_count: u16) -> Self {
        Self {
            time_signature: TimeSignature::new(time, bpm, sample_rate, channel_count),
            key,
            oscillation: 0,
            is_running: false,
        }
    }

    pub fn toggle(&mut self) {
        self.is_running = !self.is_running;
    }

    pub fn sync(&mut self, elapsed_samples: u32) {
        self.time_signature.sync(elapsed_samples);
    }
}
