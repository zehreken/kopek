use kopek::{
    oscillator::{Oscillator, A_FREQ},
    time_signature::TimeSignature,
};
use ringbuf::HeapProducer;

pub struct Temp {
    tick: f32,
    freq: f32,
    oscillator: Oscillator,
    producer: HeapProducer<f32>,
    time_4_4: TimeSignature,
    time_3_4: TimeSignature,
    time_5_4: TimeSignature,
}

impl Temp {
    pub fn new(producer: HeapProducer<f32>, sample_rate: f32) -> Result<Temp, anyhow::Error> {
        Ok(Temp {
            tick: 0.0,
            freq: A_FREQ * 16.0,
            oscillator: Oscillator::new(sample_rate),
            producer,
            time_4_4: TimeSignature::new((4, 4), 120),
            time_3_4: TimeSignature::new((3, 4), 90),
            time_5_4: TimeSignature::new((5, 4), 150),
        })
    }

    pub fn update(&mut self) {
        for _ in 0..1024 {
            if !self.producer.is_full() {
                let mut value = 0.0;
                let (show_4_4, accent) = self.time_4_4.update();
                if show_4_4 {
                    value += self.oscillator.sine(self.freq, self.tick);
                }

                let (show_3_4, accent) = self.time_3_4.update();
                if show_3_4 {
                    value += self.oscillator.sine(self.freq * 2.0, self.tick);
                }

                let (show_5_4, accent) = self.time_5_4.update();
                if show_5_4 {
                    value += self.oscillator.sine(self.freq * 2.0, self.tick);
                }

                self.producer.push(value).unwrap();
                self.tick += 1.0;
            }
        }
    }
}
