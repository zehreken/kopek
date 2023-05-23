use kopek::{oscillator::*, time_signature::TimeSignature};
use ringbuf::{HeapConsumer, HeapProducer};

use crate::view::{Input, ViewMessage};

pub struct App {
    tick: f32,
    oscillator: Oscillator,
    producer: HeapProducer<f32>,
    input_consumer: HeapConsumer<Input>,
    view_producer: HeapProducer<ViewMessage>,
    time_4_4: TimeSignature,
    time_3_4: TimeSignature,
    time_5_4: TimeSignature,
}

impl App {
    pub fn new(
        sample_rate: f32,
        producer: HeapProducer<f32>,
        input_consumer: HeapConsumer<Input>,
        view_producer: HeapProducer<ViewMessage>,
    ) -> Result<App, anyhow::Error> {
        Ok(App {
            tick: 0.0,
            oscillator: Oscillator::new(sample_rate),
            producer,
            input_consumer,
            view_producer,
            time_4_4: TimeSignature::new((4, 4), 120),
            time_3_4: TimeSignature::new((3, 4), 90),
            time_5_4: TimeSignature::new((5, 4), 75),
        })
    }

    pub fn update(&mut self) {
        for _ in 0..1024 {
            if !self.producer.is_full() {
                let mut value = 0.0;
                let (show_4_4, accent) = self.time_4_4.update();
                if show_4_4 {
                    value += self.oscillator.sine(C_FREQ * 16.0, self.tick);
                }

                let (show_3_4, accent) = self.time_3_4.update();
                if show_3_4 {
                    value += self.oscillator.square(E_FREQ * 16.0, self.tick);
                }

                let (show_5_4, accent) = self.time_5_4.update();
                if show_5_4 {
                    value += self.oscillator.triangle(G_FREQ * 16.0, self.tick);
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
    }
}
