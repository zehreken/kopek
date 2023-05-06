use crate::{metronome::Metronome, oscillator};

// The idea with this is that time_signature will use the metronome
// to count a specific time, e.g. 4/4, 3/4, 7/8 etc.
// Since metronome or bpm is required and used the same way
// by any time signature
pub struct TimeSignature {
    time: (u8, u8),
    metronome: Metronome,
}

impl TimeSignature {
    pub fn new(time: (u8, u8), bpm: u16) -> Self {
        let metronome = Metronome::new(bpm, 44100, 4);
        Self { time, metronome }
    }

    pub fn update(&mut self) -> bool {
        self.metronome.update();

        self.metronome.show_beat() && self.metronome.get_beat_index() % self.time.0 as u32 == 0
    }
}
