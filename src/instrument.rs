use crate::app::Instruction;

pub mod oscillator;

pub trait Instrument: Send {
    fn tick(&mut self) -> f32;

    fn set_sample_rate(&mut self, sample_rate: f32);

    fn apply_instruction(&mut self, instruction: Instruction);
}
