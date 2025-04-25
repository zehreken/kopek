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
pub enum Key {
    C,
    Cs,
    D,
    Ds,
    E,
    F,
    Fs,
    G,
    Gs,
    A,
    As,
    B,
    Rest,
}

pub fn get_freq(key: Key) -> f32 {
    match key {
        Key::C => C_FREQ,
        Key::Cs => CS_FREQ,
        Key::D => D_FREQ,
        Key::Ds => DS_FREQ,
        Key::E => E_FREQ,
        Key::F => F_FREQ,
        Key::Fs => FS_FREQ,
        Key::G => G_FREQ,
        Key::Gs => GS_FREQ,
        Key::A => A_FREQ,
        Key::As => AS_FREQ,
        Key::B => B_FREQ,
        Key::Rest => REST,
    }
}
