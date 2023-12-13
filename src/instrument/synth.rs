use crate::app::Instruction;
use crate::instrument::adsr::ADSR;
use crate::instrument::oscillator::{Oscillator, Waveform};
use crate::instrument::vibrato::Vibrato;
use crate::instrument::Instrument;

pub struct Synth {
    oscillator: Oscillator,
    adsr: ADSR,
    vibrato: Vibrato,
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
        }
    }
}

impl Instrument for Synth {
    fn tick(&mut self) -> f32 {
        // let og = self.oscillator.frequency_hz;
        // self.oscillator.frequency_hz += self.vibrato.tick();
        self.oscillator.current_sample_jump = self.vibrato.tick();
        let ans = self.oscillator.tick() * self.adsr.tick();
        // self.oscillator.frequency_hz = og;
        ans
    }

    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.oscillator.sample_rate = sample_rate;
        self.adsr.set_sample_rate(sample_rate);
        self.vibrato.set_sample_rate(sample_rate);
    }

    fn apply_instruction(&mut self, instruction: Instruction) {
        match instruction {
            Instruction::Waveform(w) => self.oscillator.waveform = w,
            Instruction::SetState(b) => if b { self.adsr.press() } else { self.adsr.release() },
            Instruction::SetVibrato(b) => self.vibrato.set_state(b),
            Instruction::Frequency(f) => self.oscillator.frequency_hz = f,
            _ => {}
        }
    }
}
