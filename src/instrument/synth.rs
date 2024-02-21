use crate::instruction::{InstructionKind, Status};
use crate::instrument::adsr::Adsr;
use crate::instrument::oscillator::{Oscillator, Waveform};
use crate::instrument::vibrato::Vibrato;
use crate::instrument::Instrument;

pub struct Synth {
    oscillator: Oscillator,
    adsr: Adsr,
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
            adsr: Adsr::new(0.1, 0.5, 0.5, 0.3),
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

    fn apply_instruction(&mut self, instruction: InstructionKind) -> Result<(), &'static str> {
        match instruction {
            InstructionKind::Waveform(w) => self.oscillator.waveform = w,
            InstructionKind::State(s) => match s {
                Status::On => self.adsr.press(),
                Status::Off => self.adsr.release()
            },
            InstructionKind::Vibrato(s) => match s {
                Status::On => self.vibrato.set_state(true),
                Status::Off => self.vibrato.set_state(false)
            }
            InstructionKind::Frequency(f) => self.oscillator.frequency_hz = f,
            InstructionKind::Note(u) => self.oscillator.frequency_hz = 2f32.powf((u as f32 - 60.0) / 12.0) * 440.0,
            InstructionKind::VibratoSettings { rate, depth } => self.vibrato.set_rate_and_depth(rate, depth),
            InstructionKind::AdsrSettings { a, d, s, r } => self.adsr = Adsr::new(a, d, s, r),
            _ => return Err("Illegal instruction for 'Synth'")
        }
        Ok(())
    }
}
