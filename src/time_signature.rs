use crate::metronome::Metronome;

// The idea with this is that time_signature will use the metronome
// to count a specific time, e.g. 4/4, 3/4, 7/8 etc.
// Since metronome or bpm is required and used the same way
// by any time signature
#[derive(Clone, Copy)]
pub struct TimeSignature {
    time: (u8, u8),
    metronome: Metronome,
}

impl TimeSignature {
    pub fn new(time: (u8, u8), bpm: u16, sample_rate: u32, channel_count: u16) -> Self {
        let metronome = Metronome::new(bpm, sample_rate, channel_count as u32);
        Self { time, metronome }
    }

    pub fn update(&mut self) -> (bool, bool) {
        self.metronome.update();

        (
            self.metronome.show_beat(),                                // beat
            self.metronome.get_beat_index() % self.time.0 as u32 == 0, // accent
        )
    }

    pub fn get_beat_index(&self) -> u32 {
        self.metronome.get_beat_index()
    }
}
