use crate::noise::Noise;

pub struct NoiseGenerator {
    noise: Noise,
    pub noise_type: NoiseType,
}

impl NoiseGenerator {
    pub fn new() -> Self {
        NoiseGenerator {
            noise: Noise::new(),
            noise_type: NoiseType::None,
        }
    }

    pub fn run(&mut self) -> f32 {
        match self.noise_type {
            NoiseType::None => 0.0,
            NoiseType::Random => self.noise.rand_noise(),
            NoiseType::White => self.noise.white_noise(),
        }
    }

    pub fn noise_type_mut(&mut self) -> &mut NoiseType {
        &mut self.noise_type
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum NoiseType {
    None = 0,
    Random = 1,
    White = 2,
}

impl NoiseType {
    pub fn from_u8(value: u8) -> Option<NoiseType> {
        match value {
            0 => Some(NoiseType::None),
            1 => Some(NoiseType::Random),
            2 => Some(NoiseType::White),
            _ => None,
        }
    }
}
