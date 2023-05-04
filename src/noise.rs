use rand::rngs::OsRng;
use rand::Rng;

pub struct Noise {
    rand_gen: OsRng,
}

impl Noise {
    pub fn new() -> Self {
        Self {
            rand_gen: OsRng::default(),
        }
    }

    pub fn rand_noise(&mut self) -> f32 {
        self.rand_gen.gen::<f32>() * 2.0 - 1.0
    }

    pub fn white_noise(&mut self) -> f32 {
        self.rand_gen.sample(rand_distr::StandardNormal)
    }
}
