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
            EnvelopeState::Attack => todo!(),
            EnvelopeState::Decay => todo!(),
            EnvelopeState::Sustain => todo!(),
            EnvelopeState::Release => todo!(),
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
