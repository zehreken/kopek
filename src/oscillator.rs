pub struct Oscillator {
    sample_rate: f32,
}

impl Oscillator {
    pub fn new(sample_rate: f32) -> Self {
        Self { sample_rate }
    }

    pub fn sine(&self, freq: f32, tick: u32) -> f32 {
        (tick as f32 * 2.0 * std::f32::consts::PI * freq / self.sample_rate).sin()
    }

    pub fn sawtooth(&self, freq: f32, tick: u32) -> f32 {
        let freq_incr = freq / self.sample_rate;
        let phase: f32 = (tick as f32 * freq_incr) % 1.0;
        let value = (phase - phase.floor()) - 0.5;

        value
    }

    // duty is between 0 and 1
    pub fn square(&self, freq: f32, tick: u32, duty: f32) -> f32 {
        let value = (tick as f32 * 2.0 * std::f32::consts::PI * freq / self.sample_rate).sin();
        if value > duty - 0.5 {
            0.5
        } else {
            -0.5
        }
    }

    pub fn triangle(&self, freq: f32, tick: u32) -> f32 {
        let freq_incr = freq / self.sample_rate;
        let phase: f32 = (tick as f32 * freq_incr) % 1.0;
        let value = 1.0 - 4.0 * (phase - 0.5).abs();

        value
    }
}

// This is AI generated code
// fn sawtooth_wave(freq: f32, sample_rate: f32, duration: f32) -> Vec<f32> {
//     let num_samples = (duration * sample_rate) as usize;
//     let freq_incr = freq / sample_rate;
//     let mut phase: f32 = 0.0;
//     let mut waveform = Vec::with_capacity(num_samples);

//     for i in 0..num_samples {
//         let value = 2.0 * (phase - phase.floor()) - 1.0;
//         waveform.push(value);
//         phase = (phase + freq_incr) % 1.0;
//     }

//     waveform
// }
