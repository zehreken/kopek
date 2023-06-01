use std::time::Duration;

pub struct Envelope {
    state: EnvelopeState,
}

impl Envelope {
    pub fn new() -> Self {
        Self {
            state: EnvelopeState::Idle,
        }
    }

    pub fn update(&mut self, pressed: bool, released: bool) {
        match self.state {
            EnvelopeState::Idle => self.state = EnvelopeState::Attack,
            EnvelopeState::Attack => {
                println!("Attack");
                std::thread::sleep(Duration::from_millis(100));
                self.state = EnvelopeState::Decay;
            }
            EnvelopeState::Decay => {
                println!("Decay");
                std::thread::sleep(Duration::from_millis(100));
                self.state = EnvelopeState::Sustain;
            }
            EnvelopeState::Sustain => {
                println!("Sustain");
                std::thread::sleep(Duration::from_millis(250));
                self.state = EnvelopeState::Release;
            }
            EnvelopeState::Release => {
                println!("Release");
                std::thread::sleep(Duration::from_millis(100));
                self.state = EnvelopeState::Idle;
            }
        }
    }

    pub fn attack() {}

    pub fn decay() {}

    pub fn sustain() {}

    pub fn release() {}
}

// Envelope is a basic state machine
pub enum EnvelopeState {
    Idle,
    Attack,
    Decay,
    Sustain,
    Release,
}
