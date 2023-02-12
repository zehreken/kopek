pub const C_FREQ: f32 = 261.63;
pub const D_FREQ: f32 = 293.66;
pub const E_FREQ: f32 = 329.63;
pub const F_FREQ: f32 = 349.23;
pub const G_FREQ: f32 = 392.00;
pub const A_FREQ: f32 = 440.0;
pub const B_FREQ: f32 = 493.88;

pub fn get_sine(freq: f32, tick: f32) -> f32 {
    let volume = 0.2;

    (tick * 2.0 * std::f32::consts::PI * freq / 44100.0).sin() * volume
}
