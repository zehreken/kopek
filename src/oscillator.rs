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
            frequency: 0.0,
            wave_type: WaveType::Sine,
            phase: 0.0,
        }
    }

    pub fn frequency(&self) -> f32 {
        self.frequency
    }

    pub fn set_frequency(&mut self, frequency: f32) {
        self.frequency = frequency;
    }

    pub fn wave_type(&self) -> WaveType {
        self.wave_type
    }

    pub fn set_wave_type(&mut self, wave_type: WaveType) {
        self.wave_type = wave_type;
    }

    pub fn run(&mut self) -> f32 {
        if self.frequency < f32::EPSILON {
            return 0.0;
        }
        match self.wave_type {
            WaveType::Sine => self.sine(),
            WaveType::FakeSine => self.fake_sine(),
            WaveType::Sawtooth => self.sawtooth(),
            WaveType::Square { duty } => self.square(duty),
            WaveType::Triangle => self.triangle(),
        }
    }

    pub fn sine(&mut self) -> f32 {
        let value = self.phase.sin();
        let phase_increment = 2.0 * std::f32::consts::PI * self.frequency / self.sample_rate;
        self.phase = (self.phase + phase_increment) % (2.0 * std::f32::consts::PI);

        value
    }

    // https://bmtechjournal.wordpress.com/2020/05/27/super-fast-quadratic-sinusoid-approximation/
    pub fn fake_sine(&mut self) -> f32 {
        let x = (self.phase / std::f32::consts::PI) - 1.0; // x in [-1, 1]
        let value = 4.0 * x * (1.0 - x.abs());
        let phase_increment = 2.0 * std::f32::consts::PI * self.frequency / self.sample_rate;
        self.phase = (self.phase + phase_increment) % (2.0 * std::f32::consts::PI);

        value
    }

    pub fn sawtooth(&mut self) -> f32 {
        let normalized_phase = self.phase / (2.0 * std::f32::consts::PI);
        let value = 2.0 * normalized_phase - 1.0;
        let phase_increment = 2.0 * std::f32::consts::PI * self.frequency / self.sample_rate;
        self.phase = (self.phase + phase_increment) % (2.0 * std::f32::consts::PI);

        value
    }

    // duty is between 0 and 1
    pub fn square(&mut self, duty: f32) -> f32 {
        let duty = duty.clamp(0.0, 1.0);
        let value = if self.phase < duty { 1.0 } else { -1.0 };

        let phase_increment = self.frequency / self.sample_rate;
        self.phase = (self.phase + phase_increment).fract();

        value
    }

    pub fn triangle(&mut self) -> f32 {
        let value = 1.0 - 4.0 * (self.phase - 0.5).abs();
        let phase_increment = self.frequency / self.sample_rate;
        self.phase = (self.phase + phase_increment).fract();

        value
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WaveType {
    Sine,
    FakeSine,
    Sawtooth,
    Square { duty: f32 },
    Triangle,
}

impl WaveType {
    pub fn to_u8(&self) -> u8 {
        match self {
            WaveType::Sine => 0,
            WaveType::FakeSine => 1,
            WaveType::Sawtooth => 2,
            WaveType::Square { duty: _ } => 3,
            WaveType::Triangle => 4,
        }
    }

    pub fn from_u8(value: u8) -> Option<Self> {
        match value {
            0 => Some(WaveType::Sine),
            1 => Some(WaveType::FakeSine),
            2 => Some(WaveType::Sawtooth),
            3 => Some(WaveType::Square { duty: 0.5 }), // default duty cycle
            4 => Some(WaveType::Triangle),
            _ => None,
        }
    }
}
