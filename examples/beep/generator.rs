use crate::view::Input;
use kopek::oscillator::*;
use ringbuf::{HeapConsumer, HeapProducer};

enum Oscillator {
    Sine,
    Sawtooth,
    Square,
    Triangle,
}

pub struct Generator {
    is_running: bool,
    tick: f32,
    freq: f32,
    oscillator: Oscillator,
    producer: HeapProducer<f32>,
    input_consumer: HeapConsumer<Input>,
    view_producer: HeapProducer<f32>,
}

impl Generator {
    pub fn new(
        producer: HeapProducer<f32>,
        input_consumer: HeapConsumer<Input>,
        view_producer: HeapProducer<f32>,
    ) -> Result<Generator, anyhow::Error> {
        Ok(Generator {
            is_running: false,
            tick: 0.0,
            freq: A_FREQ,
            oscillator: Oscillator::Sine,
            producer,
            input_consumer,
            view_producer,
        })
    }

    pub fn update(&mut self) {
        for _ in 0..1024 {
            if self.is_running && !self.producer.is_full() {
                let value = match self.oscillator {
                    Oscillator::Sine => kopek::oscillator::sine(self.freq, self.tick),
                    Oscillator::Sawtooth => kopek::oscillator::sawtooth(self.freq, self.tick),
                    Oscillator::Square => todo!(),
                    Oscillator::Triangle => todo!(),
                };
                // let value = kopek::wave::white_noise();
                // let value = kopek::wave::rand_noise();
                self.producer.push(value).unwrap();
                if !self.view_producer.is_full() {
                    self.view_producer.push(value).unwrap();
                }
                self.tick += 1.0;
            }
        }
        // Input
        if let Some(input) = self.input_consumer.pop() {
            match input {
                Input::Start => self.is_running = true,
                Input::Stop => self.is_running = false,
                Input::ChangeFreq(freq) => self.freq = freq,
                Input::ChangeOscillator(osc) => {
                    if osc == 0 {
                        self.oscillator = Oscillator::Sine;
                    }
                    if osc == 1 {
                        self.oscillator = Oscillator::Sawtooth;
                    }
                }
            }
        }
    }
}
