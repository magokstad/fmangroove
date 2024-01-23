use crate::app::{InstructionKind, Status};
use crate::instrument::adsr::ADSR;
use crate::instrument::oscillator::{Oscillator, Waveform};
use crate::instrument::vibrato::Vibrato;
use crate::instrument::Instrument;

pub struct Synth {
    oscillator: Oscillator,
    adsr: ADSR,
    vibrato: Vibrato,
    volume: (f32, f32),
}

impl Synth {
    pub fn new() -> Self {
        let mut osc = Oscillator::default();
        osc.is_on = true;
        osc.waveform = Waveform::Triangle;
        Self {
            oscillator: osc,
            adsr: ADSR::new(0.1, 0.5, 0.5, 0.3),
            vibrato: Vibrato::new(6.0, 0.01),
            volume: (1.0, 1.0),
        }
    }
}

impl Instrument for Synth {
    fn tick(&mut self) -> (f32, f32) {
        self.oscillator.current_sample_jump = self.vibrato.tick();
        let ans = self.oscillator.tick() * self.adsr.tick();
        (ans * self.volume.0, ans * self.volume.1)
    }

    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.oscillator.sample_rate = sample_rate;
        self.adsr.set_sample_rate(sample_rate);
        self.vibrato.set_sample_rate(sample_rate);
    }

    fn apply_instruction(&mut self, instruction: InstructionKind) {
        match instruction {
            InstructionKind::Waveform(w) => self.oscillator.waveform = w,
            InstructionKind::SetState(s) => match s {
                Status::On => self.adsr.press(),
                Status::Off => self.adsr.release()
            },
            InstructionKind::SetVibrato(s) => match s {
                Status::On => self.vibrato.set_state(false),
                Status::Off => self.vibrato.set_state(false)
            }
            InstructionKind::Frequency(f) => self.oscillator.frequency_hz = f,
            _ => {}
        }
    }
}
