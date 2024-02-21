use crate::instruction::{InstructionKind, Status};
use crate::instrument::Instrument;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Waveform {
    Sine,
    Square,
    Saw,
    Triangle,
}

pub struct Oscillator {
    pub sample_rate: f32,
    pub waveform: Waveform,
    pub current_sample_index: f32,
    pub current_sample_jump: f32,
    pub frequency_hz: f32,
    pub is_on: bool,
}

impl Oscillator {
    pub fn default() -> Self {
        Self {
            sample_rate: 0.0,
            waveform: Waveform::Sine,
            current_sample_index: 0.0,
            current_sample_jump: 1.0,
            frequency_hz: 220.0,
            is_on: false,
        }
    }

    fn advance_sample(&mut self) {
        self.current_sample_index = (self.current_sample_index + self.current_sample_jump) % (self.sample_rate / self.frequency_hz)
        // self.current_sample_index = (self.current_sample_index + self.current_sample_jump) % (two_pi * self.frequency_hz);
        // self.current_sample_index += 1.0;
        // self.current_sample_index += self.current_sample_jump;
    }

    pub fn set_waveform(&mut self, waveform: Waveform) {
        self.waveform = waveform;
    }

    fn calculate_sine_output_from_freq(&self, freq: f32) -> f32 {
        let two_pi = 2.0 * std::f32::consts::PI;
        (self.current_sample_index * freq * two_pi / self.sample_rate).sin()
    }

    fn is_multiple_of_freq_above_nyquist(&self, multiple: f32) -> bool {
        self.frequency_hz * multiple > self.sample_rate / 2.0
    }

    fn sine_wave(&mut self) -> f32 {
        self.calculate_sine_output_from_freq(self.frequency_hz)
    }

    fn generative_waveform(
        &mut self,
        harmonic_index_increment: i32,
        gain_exponent: f32,
    ) -> f32 {
        let mut output = 0.0;
        let mut i = 1;
        while !self.is_multiple_of_freq_above_nyquist(i as f32) {
            let gain = 1.0 / (i as f32).powf(gain_exponent);
            output += gain
                * self
                    .calculate_sine_output_from_freq(self.frequency_hz * i as f32);
            i += harmonic_index_increment;
        }
        output
    }

    fn square_wave(&mut self) -> f32 {
        self.generative_waveform(2, 1.0)
    }

    fn saw_wave(&mut self) -> f32 {
        self.generative_waveform(1, 1.0)
    }

    fn triangle_wave(&mut self) -> f32 {
        self.generative_waveform(2, 2.0)
    }

    pub fn tick(&mut self) -> f32 {
        self.advance_sample();
        if !self.is_on {
            return 0.0;
        }
        match self.waveform {
            Waveform::Sine => self.sine_wave(),
            Waveform::Square => self.square_wave(),
            Waveform::Saw => self.saw_wave(),
            Waveform::Triangle => self.triangle_wave(),
        }
    }
}

impl Instrument for Oscillator {
    fn tick(&mut self) -> (f32, f32) {
        let x = self.tick();
        (x,x)
    }

    fn set_sample_rate(&mut self, sample_rate: f32) {
        self.sample_rate = sample_rate
    }

    fn apply_instruction(&mut self, instruction: InstructionKind) -> Result<(), &'static str> {
        match instruction {
            InstructionKind::Waveform(w) => self.waveform = w,
            InstructionKind::Frequency(f) => self.frequency_hz = f,
            InstructionKind::Note(u) => self.frequency_hz = 2f32.powf((u as f32 - 60.0) / 12.0) * 440.0,
            InstructionKind::State(s) => match s {
                Status::On => self.is_on = true,
                Status::Off => self.is_on = false
            },
            _ => return Err("Illegal instruction for 'Oscillator'")
        }
        Ok(())
    }
}
