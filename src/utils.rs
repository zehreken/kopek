// Frequencies from this page
// https://pages.mtu.edu/~suits/notefreqs.html
pub const C_FREQ: f32 = 16.35;
pub const D_FREQ: f32 = 18.35;
pub const E_FREQ: f32 = 20.60;
pub const F_FREQ: f32 = 21.83;
pub const G_FREQ: f32 = 24.50;
pub const A_FREQ: f32 = 27.50;
pub const B_FREQ: f32 = 30.87;

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
