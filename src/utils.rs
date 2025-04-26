use std::fmt::Display;

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

impl Display for Key {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Key::C => write!(f, "C"),
            Key::Cs => write!(f, "C#"),
            Key::D => write!(f, "D"),
            Key::Ds => write!(f, "D#"),
            Key::E => write!(f, "E"),
            Key::F => write!(f, "F"),
            Key::Fs => write!(f, "F#"),
            Key::G => write!(f, "G"),
            Key::Gs => write!(f, "G#"),
            Key::A => write!(f, "A"),
            Key::As => write!(f, "A#"),
            Key::B => write!(f, "B"),
            Key::Rest => write!(f, "_"),
        }
    }
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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Octave {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
}

pub const OCTAVES: [(&str, Octave); 5] = [
    ("First", Octave::First),
    ("Second", Octave::Second),
    ("Third", Octave::Third),
    ("Fourth", Octave::Fourth),
    ("Fifth", Octave::Fifth),
];

impl Display for Octave {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Octave::First => write!(f, "1"),
            Octave::Second => write!(f, "2"),
            Octave::Third => write!(f, "3"),
            Octave::Fourth => write!(f, "4"),
            Octave::Fifth => write!(f, "5"),
        }
    }
}

pub const KEYS: [(&str, Key); 13] = [
    ("C", Key::C),
    ("Cs", Key::Cs),
    ("D", Key::D),
    ("Ds", Key::Ds),
    ("E", Key::E),
    ("F", Key::F),
    ("Fs", Key::Fs),
    ("G", Key::G),
    ("Gs", Key::Gs),
    ("A", Key::A),
    ("As", Key::As),
    ("B", Key::B),
    ("_", Key::Rest),
];
