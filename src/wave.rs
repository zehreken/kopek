pub const A_FREQ: f32 = 440.0;
pub const C_FREQ: f32 = 523.25;

pub fn get_sine(tick: f32) -> f32 {
    let freq = A_FREQ;
    let volume = 0.2;

    (tick * 2.0 * std::f32::consts::PI * freq / 44100.0).sin() * volume
}
