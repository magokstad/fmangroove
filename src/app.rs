use std::collections::{HashMap, HashSet};
use crate::instruction_handler::InstructionHandler;
use crate::instrument::oscillator::{Oscillator, Waveform};
use crate::instrument::synth::Synth;
use crate::instrument::Instrument;

pub struct App {
    // FIXME: temporary pubs
    pub instruments: Vec<Box<dyn Instrument>>,
    pub instructions: InstructionHandler,
    delay: u16,
    tick: u128,
    playing: bool
}

impl App {
    pub fn new() -> Self {
        Self {
            instruments: vec![
                Box::new(Synth::new()),
                Box::new(Oscillator::default())
            ],
            instructions: InstructionHandler::new(),
            delay: 125,
            tick: 0,
            playing: false,
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

    pub fn play(&mut self) {
        self.playing = true
    }

    pub fn pause(&mut self) {
        self.playing = false
    }

    pub fn reset(&mut self) {
        self.tick = 0
    }

    pub fn tick_all(&mut self) -> (f32, f32) {
        // Temporary pausing
        if !self.playing {
            return (0.0, 0.0);
        }

        // TODO: instruction handling here
        // TODO: What if illegal instruction?
        // TODO: maybe give instruments a unique UUID??
        for i in 0..self.instruments.len() {
            for instruction in self.instructions.get(i as u128, self.tick) {
                self.instruments.get_mut(i).unwrap().apply_instruction(instruction);
            }
        }
        // if let Some(set) = self.instructions.get(&self.tick) {
        //     for instruction in set {
        //         if let Some(instrument) = self.instruments.get_mut(instruction.target as usize) {
        //             instrument.apply_instruction(instruction.kind);
        //         }
        //     }
        // }
        self.tick = self.tick.checked_add(1).unwrap_or(u128::MAX);

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