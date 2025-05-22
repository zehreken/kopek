use rand::rngs::ThreadRng;
use rand::Rng;

pub struct Noise {
    rand_gen: ThreadRng,
}

impl Noise {
    pub fn new() -> Self {
        Self {
            rand_gen: rand::rng(),
        }
    }

    pub fn rand_noise(&mut self) -> f32 {
        self.rand_gen.random::<f32>() * 2.0 - 1.0
    }

    pub fn white_noise(&mut self) -> f32 {
        self.rand_gen.sample(rand_distr::StandardNormal)
    }
}
