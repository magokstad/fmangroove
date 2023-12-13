use crate::app::App;
use cpal::traits::StreamTrait;
use std::sync::{Arc, Mutex};

mod app;
mod audio;
mod instrument;
mod view;

fn main() -> anyhow::Result<()> {
    let app = Arc::new(Mutex::new(App::new()));
    let stream = audio::stream_setup_for(app.clone()).expect("bad setup");

    stream.play().expect("can't play");

    view::tui(app.clone()).expect("tui error");

    Ok(())
}
