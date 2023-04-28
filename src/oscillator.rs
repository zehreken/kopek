use rand::prelude::*;

// Frequencies from this page
// https://pages.mtu.edu/~suits/notefreqs.html
pub const C_FREQ: f32 = 16.35;
pub const D_FREQ: f32 = 18.35;
pub const E_FREQ: f32 = 20.60;
pub const F_FREQ: f32 = 21.83;
pub const G_FREQ: f32 = 24.50;
pub const A_FREQ: f32 = 27.50;
pub const B_FREQ: f32 = 30.87;

pub struct Oscillator {
    sample_rate: f32,
}

impl Oscillator {
    pub fn new(sample_rate: f32) -> Self {
        Self { sample_rate }
    }

    pub fn sine(&self, freq: f32, tick: f32) -> f32 {
        (tick * 2.0 * std::f32::consts::PI * freq / self.sample_rate).sin()
    }

    pub fn sawtooth(&self, freq: f32, tick: f32) -> f32 {
        let freq_incr = freq / self.sample_rate;
        let phase: f32 = (tick * freq_incr) % 1.0;
        let value = 2.0 * (phase - phase.floor()) - 1.0;

        value
    }

    pub fn square(&self, freq: f32, tick: f32) -> f32 {
        todo!("not implemented")
    }

    pub fn triangle(&self, freq: f32, tick: f32) -> f32 {
        todo!("not implemented")
    }
}

// pub fn sine(freq: f32, tick: f32) -> f32 {
//     // let volume = 0.2; // volume should not be here, also hard coded sample rate is not good

//     (tick * 2.0 * std::f32::consts::PI * freq / SAMPLE_RATE).sin() // * volume
// }

/// This is AI generated code
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

// pub fn sawtooth(freq: f32, tick: f32) -> f32 {
//     let freq_incr = freq / SAMPLE_RATE;
//     let phase: f32 = (tick * freq_incr) % 1.0;
//     let value = 2.0 * (phase - phase.floor()) - 1.0;

//     value
// }

pub fn rand_noise() -> f32 {
    rand::thread_rng().gen::<f32>() * 2.0 - 1.0
}

pub fn white_noise() -> f32 {
    rand::thread_rng().sample(rand_distr::StandardNormal)
}
