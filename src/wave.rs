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
    rand::thread_rng().gen::<f32>() * 2.0 - 1.0
}

pub fn white_noise() -> f32 {
    rand::thread_rng().sample(rand_distr::StandardNormal)
}
