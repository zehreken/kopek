use std::time::Duration;

const ATTACK_DURATION: f32 = 0.5 * 44100.0; // Second * Sample rate
const DECAY_DURATION: f32 = 0.5 * 44100.0;
const RELEASE_DURATION: f32 = 1.0 * 44100.0;

pub struct Envelope {
    state: EnvelopeState,
    volume: f32,
    tick: f32,
}

impl Envelope {
    pub fn new() -> Self {
        Self {
            state: EnvelopeState::Idle,
            volume: 0.0,
            tick: 0.0,
        }
    }

    pub fn press(&mut self) {
        self.tick = 0.0;
        self.state = EnvelopeState::Attack;
    }

    pub fn release(&mut self) {
        // self.state = EnvelopeState::Release;
    }

    // Envelope time should be based on hardware tick, not the cpu timer
    pub fn update(&mut self) -> f32 {
        match self.state {
            EnvelopeState::Attack => {
                // println!("Attack");
                self.tick += 1.0;
                self.volume = lerp(0.0, 1.0, self.tick / ATTACK_DURATION);
                if self.tick >= ATTACK_DURATION {
                    self.tick = 0.0;
                    self.state = EnvelopeState::Decay
                }
            }
            EnvelopeState::Decay => {
                // println!("Decay");
                self.tick += 1.0;
                self.volume = lerp(1.0, 0.8, self.tick / DECAY_DURATION);
                if self.tick >= DECAY_DURATION {
                    self.tick = 0.0;
                    self.state = EnvelopeState::Release;
                }
            }
            EnvelopeState::Sustain => {
                print!("Sustain");
            }
            EnvelopeState::Release => {
                // println!("Release");
                self.volume = lerp(0.8, 0.0, self.tick / RELEASE_DURATION);
                self.tick += 1.0;
                if self.tick >= RELEASE_DURATION {
                    self.state = EnvelopeState::Idle;
                }
            }
            EnvelopeState::Idle => (),
        }

        self.volume
    }

    // fn attack() {}

    // fn decay() {}

    // fn sustain() {}

    // fn release() {}
}

fn lerp(v0: f32, v1: f32, t: f32) -> f32 {
    return v0 + t * (v1 - v0);
}

// Envelope is a basic state machine
#[derive(Debug)]
pub enum EnvelopeState {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}
