use kopek::{oscillator::*, time_signature::TimeSignature};
use ringbuf::{HeapConsumer, HeapProducer};

use crate::view::{Input, ViewMessage};
pub const BEAT_COUNT: usize = 3;

pub struct App {
    tick: f32,
    oscillator: Oscillator,
    producer: HeapProducer<f32>,
    input_consumer: HeapConsumer<Input>,
    view_producer: HeapProducer<ViewMessage>,
    time_4_4: TimeSignature,
    time_3_4: TimeSignature,
    time_5_4: TimeSignature,
    beats: [ExampleBeat; BEAT_COUNT],
}

impl App {
    pub fn new(
        sample_rate: f32,
        channel_count: u16,
        producer: HeapProducer<f32>,
        input_consumer: HeapConsumer<Input>,
        view_producer: HeapProducer<ViewMessage>,
    ) -> Result<App, anyhow::Error> {
        let example_beat = ExampleBeat::new(sample_rate as u32, channel_count);
        Ok(App {
            tick: 0.0,
            oscillator: Oscillator::new(sample_rate),
            producer,
            input_consumer,
            view_producer,
            time_4_4: TimeSignature::new((4, 4), 120, sample_rate as u32, channel_count),
            time_3_4: TimeSignature::new((3, 4), 90, sample_rate as u32, channel_count),
            time_5_4: TimeSignature::new((5, 4), 75, sample_rate as u32, channel_count),
            beats: [example_beat; BEAT_COUNT],
        })
    }

    pub fn update(&mut self) {
        for _ in 0..1024 {
            if !self.producer.is_full() {
                let mut value = 0.0;
                // let (show_4_4, accent) = self.time_4_4.update();
                // if show_4_4 {
                //     value += self.oscillator.sine(C_FREQ * 16.0, self.tick);
                // }

                // let (show_3_4, accent) = self.time_3_4.update();
                // if show_3_4 {
                //     value += self.oscillator.square(E_FREQ * 16.0, self.tick);
                // }

                // let (show_5_4, accent) = self.time_5_4.update();
                // if show_5_4 {
                //     value += self.oscillator.triangle(G_FREQ * 16.0, self.tick);
                // }

                for beat in &mut self.beats {
                    let (show, accent) = beat.time_signature.update();
                    if show {
                        value += self.oscillator.sine(beat.key * 16.0, self.tick);
                    }
                }

                self.producer.push(value).unwrap();
                self.tick += 1.0;
            }
        }

        if self.view_producer.free_len() >= 3 {
            self.view_producer
                .push(ViewMessage::Beat4_4(self.time_4_4.get_beat_index()))
                .unwrap();
            self.view_producer
                .push(ViewMessage::Beat3_4(self.time_3_4.get_beat_index()))
                .unwrap();
            self.view_producer
                .push(ViewMessage::Beat5_4(self.time_5_4.get_beat_index()))
                .unwrap();
        }

        if self.view_producer.free_len() >= 5 {
            let mut time_index = 0;
            for time in self.beats {
                self.view_producer
                    .push(ViewMessage::Beat(
                        time_index,
                        time.time_signature.get_beat_index(),
                    ))
                    .unwrap();
                time_index += 1;
            }
        }
    }
}

#[derive(Clone, Copy)]
struct ExampleBeat {
    time_signature: TimeSignature,
    key: f32,
    is_running: bool,
}

impl ExampleBeat {
    pub fn new(sample_rate: u32, channel_count: u16) -> Self {
        Self {
            time_signature: TimeSignature::new((4, 4), 120, sample_rate, channel_count),
            key: C_FREQ,
            is_running: false,
        }
    }

    pub fn toggle(&mut self) {
        self.is_running = !self.is_running;
    }
}
