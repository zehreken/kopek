pub struct Oscillator {
    sample_rate: f32,
    frequency: f32,
    wave_type: WaveType,
    phase: f32,
}

impl Oscillator {
    pub fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            frequency: 440.0,
            wave_type: WaveType::Sine,
            phase: 0.0,
        }
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }

    pub fn set_wave_type(&mut self, wave_type: WaveType) {
        self.wave_type = wave_type;
    }

    pub fn run(&mut self, tick: u32) -> f32 {
        match self.wave_type {
            // WaveType::Sine => self.sine(tick),
            WaveType::Sine => self.sine_two(),
            WaveType::Sawtooth => self.sawtooth(tick),
            WaveType::Square { duty } => self.square(tick, duty),
            WaveType::Triangle => self.triangle(tick),
        }
    }

    pub fn sine(&self, tick: u32) -> f32 {
        (tick as f32 * 2.0 * std::f32::consts::PI * self.frequency / self.sample_rate).sin()
    }

    pub fn sine_two(&mut self) -> f32 {
        let value = self.phase.sin();
        let phase_increment = 2.0 * std::f32::consts::PI * self.frequency / self.sample_rate;
        self.phase += phase_increment % (2.0 * std::f32::consts::PI);
        value
    }

    pub fn sawtooth(&self, tick: u32) -> f32 {
        let freq_incr = self.frequency / self.sample_rate;
        let phase: f32 = (tick as f32 * freq_incr) % 1.0;
        let value = (phase - phase.floor()) - 0.5;

        value
    }

    // duty is between 0 and 1
    pub fn square(&self, tick: u32, duty: f32) -> f32 {
        let value =
            (tick as f32 * 2.0 * std::f32::consts::PI * self.frequency / self.sample_rate).sin();
        if value > duty - 0.5 {
            0.5
        } else {
            -0.5
        }
    }

    pub fn triangle(&self, tick: u32) -> f32 {
        let freq_incr = self.frequency / self.sample_rate;
        let phase: f32 = (tick as f32 * freq_incr) % 1.0;
        let value = 1.0 - 4.0 * (phase - 0.5).abs();

        value
    }
}

pub enum WaveType {
    Sine,
    Sawtooth,
    Square { duty: f32 },
    Triangle,
}

// This is AI generated code
// fn sawtooth_wave(freq: f32, sample_rate: f32, duration: f32) -> Vec<f32> {
//     let num_samples = (duration * sample_rate) as usize;
//     let freq_incr = freq / sample_rate;
//     let mut phase: f32 = 0.0;
//     let mut waveform = Vec::with_capacity(num_samples);

//     for i in 0..num_samples {
//         let value = 2.0 * (phase - phase.floor()) - 1.0;
//         waveform.push(value);
//         phase = (phase + freq_incr) % 1.0;
//     }

//     waveform
// }
