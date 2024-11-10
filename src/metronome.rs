#[derive(Debug)]
pub struct Metronome {
    pub is_running: bool,
    beat_index: u32,
    sample_count: u32,
    tick_period: f32,
    show_beat: bool,
}

impl Metronome {
    pub fn new(bpm: u16, sample_rate: u32, channel_count: u32) -> Self {
        let tick_period = (sample_rate * channel_count * 60) as f32 / bpm as f32;
        Self {
            is_running: false,
            beat_index: 0,
            sample_count: 0,
            tick_period,
            show_beat: false,
        }
    }

    // update can be called from the main thread
    pub fn update(&mut self, elapsed_samples: u32) {
        self.sample_count = elapsed_samples;

        let remainder = self.sample_count % self.tick_period as u32;
        self.show_beat = remainder > 0 && remainder < 8192;
        self.beat_index = self.sample_count / self.tick_period as u32;
    }

    // Current number of beats played, similar to elapsed time
    pub fn get_beat_index(&self) -> u32 {
        self.beat_index
    }

    // Used to make a sound or visualize
    pub fn show_beat(&self) -> bool {
        self.show_beat
    }
}
