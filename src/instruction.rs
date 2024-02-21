use std::hash::Hash;
use crate::instrument::oscillator::Waveform;
use crate::util::ParseAt;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum Status { On, Off }

#[derive(Copy, Clone, PartialEq, Debug)]
pub enum InstructionKind {
    Waveform(Waveform),
    Frequency(f32),
    // TODO: Notes should probably be something other than just a number
    Note(u16),
    State(Status),
    Vibrato(Status),
    VibratoSettings{
        rate: f32,
        depth: f32
    },
    AdsrSettings{
        a: f32,
        d: f32,
        s: f32,
        r: f32
    },
}

impl InstructionKind {
    pub fn parse(s: String) -> Result<Self, String> {
        let split = s.split_whitespace();
        let args = split.collect::<Vec<&str>>();

        let mut kind = None;

        match *args.get(0).unwrap_or(&"") {
            "form" => {
                if let Some(w) = match *args.get(1).unwrap_or(&"") {
                    "saw" => Some(Waveform::Saw),
                    "square" => Some(Waveform::Square),
                    "tri" => Some(Waveform::Triangle),
                    "sine" => Some(Waveform::Sine),
                    _ => None
                } {
                    kind = Some(InstructionKind::Waveform(w));
                }
            },
            "freq" => {
                if let Ok(f) = args.parse_at::<f32>(1) {
                    kind = Some(InstructionKind::Frequency(f));
                }
            },
            "note" => {
                if let Ok(n) = args.parse_at::<u16>(1) {
                    kind = Some(InstructionKind::Note(n));
                }
            },
            "state" => {
                if let Some(n) = match *args.get(1).unwrap_or(&"") {
                    "on" => Some(Status::On),
                    "off" => Some(Status::On),
                    _ => None
                } {
                    kind = Some(InstructionKind::State(n));
                }
            },
            "vib" => {
                match *args.get(1).unwrap_or(&"") {
                    "on" => kind = Some(InstructionKind::Vibrato(Status::On)),
                    "off" => kind = Some(InstructionKind::Vibrato(Status::Off)),
                    "opt" => {
                        if let (Ok(r), Ok(d)) = (
                            args.parse_at::<f32>(1),
                            args.parse_at::<f32>(2),
                        ) {
                            kind = Some(InstructionKind::VibratoSettings {
                                rate: r,
                                depth: d
                            });
                        }
                    }
                    other => return Err(format!("Unknown option for command vib '{}'", other))
                }
            },
            "adsr" => {
                if let (Ok(a), Ok(d), Ok(s), Ok(r)) = (
                    args.parse_at::<f32>(1),
                    args.parse_at::<f32>(2),
                    args.parse_at::<f32>(3),
                    args.parse_at::<f32>(4),
                ) {
                    kind = Some(InstructionKind::AdsrSettings { a, d, s, r });
                }
            },
            other => return Err(format!("Unknown instruction '{}'", other))
        }

        match kind {
            Some(k) => Ok(k),
            None => Err(format!("Bad arguments for instruction '{}'", s))
        }
    }
}