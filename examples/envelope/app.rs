use kopek::{
    envelope::*,
    oscillator::*,
    utils::{C_FREQ, E_FREQ, G_FREQ},
};
use ringbuf::{HeapConsumer, HeapProducer};

use crate::view::{Input, ViewMessage};

pub struct App {
    tick: f32,
    oscillator: Oscillator,
    producer: HeapProducer<f32>,
    input_consumer: HeapConsumer<Input>,
    view_producer: HeapProducer<ViewMessage>,
    envelope: Envelope,
}

impl App {
    pub fn new(
        sample_rate: f32,
        channel_count: u16,
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
            envelope: Envelope::new(sample_rate, channel_count),
        })
    }

    pub fn update(&mut self) {
        for _ in 0..1024 {
            if !self.producer.is_full() {
                let mut value = 0.0;
                self.oscillator.set_frequency(C_FREQ * 4.0);
                value += self.oscillator.sine();

                value *= self.envelope.update();

                self.producer.push(value).unwrap();
                self.tick += 1.0;
            }
        }

        while let Some(message) = self.input_consumer.pop() {
            match message {
                Input::Pressed => self.envelope.press(),
                Input::Released => self.envelope.release(),
            }
        }

        if self.view_producer.free_len() > 0 {
            self.view_producer
                .push(ViewMessage::State(self.envelope.state()))
                .unwrap();
        }
    }
}
