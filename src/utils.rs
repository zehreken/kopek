// Frequencies from this page
// https://gist.github.com/nvictor/7b4ab7070e210bc1306356f037226dd9
pub const C_FREQ: f32 = 16.35;
pub const CS_FREQ: f32 = 17.32;
pub const D_FREQ: f32 = 18.35;
pub const DS_FREQ: f32 = 19.45;
pub const E_FREQ: f32 = 20.60;
pub const F_FREQ: f32 = 21.83;
pub const FS_FREQ: f32 = 23.12;
pub const G_FREQ: f32 = 24.50;
pub const GS_FREQ: f32 = 25.96;
pub const A_FREQ: f32 = 27.50;
pub const AS_FREQ: f32 = 29.14;
pub const B_FREQ: f32 = 30.87;
pub const REST: f32 = 0.0;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Keys {
    C,
    D,
    E,
    F,
    G,
    A,
    B,
}

pub fn get_freq(key: Keys) -> f32 {
    match key {
        Keys::C => C_FREQ,
        Keys::D => D_FREQ,
        Keys::E => E_FREQ,
        Keys::F => F_FREQ,
        Keys::G => G_FREQ,
        Keys::A => A_FREQ,
        Keys::B => B_FREQ,
    }
}
