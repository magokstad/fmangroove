use crate::app::InstructionKind;

mod adsr;
pub mod oscillator;
pub mod synth;
mod vibrato;

pub trait Instrument: Send {
    fn tick(&mut self) -> (f32, f32);

    fn set_sample_rate(&mut self, sample_rate: f32);

    fn apply_instruction(&mut self, instruction: InstructionKind);
}
