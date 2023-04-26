use kopek::wave::*;
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
    input_consumer: HeapConsumer<(u8, u8)>,
    view_producer: HeapProducer<f32>,
}

impl Generator {
    pub fn new(
        producer: HeapProducer<f32>,
        input_consumer: HeapConsumer<(u8, u8)>,
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
            if !self.producer.is_full() {
                let value = match self.oscillator {
                    Oscillator::Sine => kopek::wave::sine(self.freq, self.tick),
                    Oscillator::Sawtooth => kopek::wave::sawtooth(self.freq, self.tick),
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
        if let Some((input, octave)) = self.input_consumer.pop() {
            let octave_factor = 2_u8.pow(octave as u32) as f32;
            if input == 0 {
                self.freq = C_FREQ * octave_factor;
            }
            if input == 1 {
                self.freq = D_FREQ * octave_factor;
            }
            if input == 2 {
                self.freq = E_FREQ * octave_factor;
            }
            if input == 3 {
                self.freq = F_FREQ * octave_factor;
            }
            if input == 4 {
                self.freq = G_FREQ * octave_factor;
            }
            if input == 5 {
                self.freq = A_FREQ * octave_factor;
            }
            if input == 6 {
                self.freq = B_FREQ * octave_factor;
            }
            if input == 7 {
                self.freq = C_FREQ * octave_factor * 2.0;
            }
            if input == 8 {
                self.oscillator = Oscillator::Sine;
            }
            if input == 9 {
                self.oscillator = Oscillator::Sawtooth;
            }
        }
    }
}
