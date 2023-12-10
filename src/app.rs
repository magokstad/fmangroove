use crate::instrument::Instrument;
use crate::instrument::oscillator::{Oscillator, Waveform};

pub struct App {
    pub instruments: Vec<Box<dyn Instrument>>,
    instructions: Vec<Vec<Instruction>>,
    delay: u16,
}

impl App {
    pub fn new() -> Self {
        Self {
            instruments: vec![Box::new(Oscillator::default())],
            instructions: vec![],
            delay: 125,
        }
    }

    pub fn get_delay(&self) -> u16 {
        self.delay
    }

    pub fn set_delay(&mut self, delay: u16) {
        self.delay = delay;
    }

    pub fn get_bpm(&self) -> u16 {
        return 15000 / self.delay
    }

    pub fn set_bpm(&mut self, bpm: u16) {
        self.delay = 15000 / bpm
    }

    pub fn set_sample_rates(&mut self, sample_rate: f32) {
        self.instruments.iter_mut().for_each( |it| {
            it.set_sample_rate(sample_rate);
        })
    }

    pub fn tick_all(&mut self) -> f32 {
        // TODO: find cleaner way to handle amplitude
        self.instruments.iter_mut().fold(0.0, |acc, it| { acc + it.tick() }) / 8.0
    }
}

pub enum Instruction {
    Waveform(Waveform),
    Frequency(f32),
    // TODO: Notes should probably be something other than just a number
    Note(u16),
    On,
    Off
}

