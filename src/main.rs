use std::sync::{Arc, Mutex};
use cpal::traits::StreamTrait;
use crate::app::{App, Oscillator, Waveform};

mod view;
mod app;
mod audio;

fn main() -> anyhow::Result<()> {
    let x = Arc::new(Mutex::new(
        App {
            oscillator: Oscillator {
                sample_rate: 0.0,
                waveform: Waveform::Sine,
                current_sample_index: 0.0,
                frequency_hz: 220.0,
                is_on: true,
        }}));
    let stream = audio::stream_setup_for(x.clone()).expect("bad setup");
    stream.play().expect("can't play");
    view::tui(x.clone()).expect("tui error");
    Ok(())
}