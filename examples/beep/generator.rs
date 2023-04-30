use crate::view::Input;
use kopek::oscillator::*;
use ringbuf::{HeapConsumer, HeapProducer};

pub enum OscillatorType {
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
    oscillator_type: OscillatorType,
    producer: HeapProducer<f32>,
    input_consumer: HeapConsumer<Input>,
    view_producer: HeapProducer<f32>,
}

impl Generator {
    pub fn new(
        producer: HeapProducer<f32>,
        input_consumer: HeapConsumer<Input>,
        view_producer: HeapProducer<f32>,
        sample_rate: f32,
    ) -> Result<Generator, anyhow::Error> {
        Ok(Generator {
            is_running: false,
            tick: 0.0,
            freq: A_FREQ,
            oscillator: Oscillator::new(sample_rate),
            oscillator_type: OscillatorType::Sine,
            producer,
            input_consumer,
            view_producer,
        })
    }

    pub fn update(&mut self) {
        for _ in 0..1024 {
            if self.is_running && !self.producer.is_full() {
                let value = match self.oscillator_type {
                    OscillatorType::Sine => self.oscillator.sine(self.freq, self.tick),
                    OscillatorType::Sawtooth => self.oscillator.sawtooth(self.freq, self.tick),
                    OscillatorType::Square => self.oscillator.square(self.freq, self.tick),
                    OscillatorType::Triangle => self.oscillator.triangle(self.freq, self.tick),
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
                        self.oscillator_type = OscillatorType::Sine;
                    }
                    if osc == 1 {
                        self.oscillator_type = OscillatorType::Sawtooth;
                    }
                    if osc == 2 {
                        self.oscillator_type = OscillatorType::Square;
                    }
                    if osc == 3 {
                        self.oscillator_type = OscillatorType::Triangle;
                    }
                }
            }
        }
    }
}
