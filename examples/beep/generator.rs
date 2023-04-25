use kopek::wave::*;
use ringbuf::{HeapConsumer, HeapProducer};

pub struct Generator {
    is_running: bool,
    tick: f32,
    freq: f32,
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
            producer,
            input_consumer,
            view_producer,
        })
    }

    pub fn update(&mut self) {
        for _ in 0..1024 {
            if !self.producer.is_full() {
                // let value = kopek::wave::sawtooth(freq, tick);
                let value = kopek::wave::sine(self.freq, self.tick);
                // let value = kopek::wave::white_noise();
                // let value = kopek::wave::rand_noise();
                self.producer.push(value).unwrap();
                if !self.view_producer.is_full() {
                    self.view_producer.push(value).unwrap();
                }
                self.tick += 1.0;
            }
        }
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
        }
    }
}
