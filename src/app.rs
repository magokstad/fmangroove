use std::collections::HashMap;
use crate::instrument::oscillator::{Waveform};
use crate::instrument::synth::Synth;
use crate::instrument::Instrument;

pub struct App {
    pub instruments: Vec<Box<dyn Instrument>>,
    instructions: HashMap<u128, Vec<Instruction>>,
    delay: u16,
    tick: u128,
}

impl App {
    pub fn new() -> Self {
        Self {
            instruments: vec![Box::new(Synth::new())],
            instructions: HashMap::new(),
            delay: 125,
            tick: 0
        }
    }

    pub fn get_delay(&self) -> u16 {
        self.delay
    }

    pub fn set_delay(&mut self, delay: u16) {
        self.delay = delay;
    }

    pub fn get_bpm(&self) -> u16 {
        return 15000 / self.delay;
    }

    pub fn set_bpm(&mut self, bpm: u16) {
        self.delay = 15000 / bpm
    }

    pub fn set_sample_rates(&mut self, sample_rate: f32) {
        self.instruments.iter_mut().for_each(|it| {
            it.set_sample_rate(sample_rate);
        })
    }

    pub fn tick_all(&mut self) -> (f32, f32) {
        // TODO: instruction handling here


        // Audio handling
        let (mut left, mut right) = (0.0, 0.0);
        for inst in self.instruments.iter_mut() {
            let (l,r) = inst.tick();
            left += l;
            right += r;
        }
        // TODO: find cleaner way to handle amplitude
        (left / 8.0, right / 8.0)
    }
}

pub enum Instruction {
    Waveform(Waveform),
    Frequency(f32),
    // TODO: Notes should probably be something other than just a number
    Note(u16),
    SetState(bool),
    SetVibrato(bool),
}
