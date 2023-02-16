use rand::prelude::*;
pub const C_FREQ: f32 = 261.63;
pub const D_FREQ: f32 = 293.66;
pub const E_FREQ: f32 = 329.63;
pub const F_FREQ: f32 = 349.23;
pub const G_FREQ: f32 = 392.00;
pub const A_FREQ: f32 = 440.0;
pub const B_FREQ: f32 = 493.88;

pub fn sine(freq: f32, tick: f32) -> f32 {
    // let volume = 0.2; // volume should not be here, also hard coded sample rate is not good

    (tick * 2.0 * std::f32::consts::PI * freq / 44100.0).sin() // * volume
}

pub fn saw(freq: f32, tick: f32) -> f32 {
    let base = -1.0;
    let sample_rate = 44100.0;
    let p = freq / sample_rate;
    let mut value = base + p * (tick % 200.0);
    if value >= 1.0 {
        value -= 2.0;
    }
    value
}

pub fn rand_noise() -> f32 {
    rand::thread_rng().gen()
}

pub fn white_noise() -> f32 {
    rand::thread_rng().sample(rand_distr::StandardNormal)
}
