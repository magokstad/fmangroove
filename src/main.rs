use std::sync::{Arc, Mutex};
use cpal::traits::StreamTrait;
use crate::app::App;

mod view;
mod app;
mod audio;
mod tui_elements;

fn main() -> anyhow::Result<()> {
    let app = Arc::new(Mutex::new(App:: new()));
    let stream = audio::stream_setup_for(app.clone()).expect("bad setup");

    stream.play().expect("can't play");

    view::tui(app.clone()).expect("tui error");

    Ok(())
}