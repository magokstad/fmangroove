use std::io::{stdout, Write, Result};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use crossterm::{ExecutableCommand, cursor, style, Command, QueueableCommand, queue};
use crossterm::terminal::{Clear, ClearType, self, enable_raw_mode, disable_raw_mode};
use crossterm::event::{Event, self, KeyCode};
use crate::app::{App, Waveform};

#[derive(Copy, Clone)]
enum TuiMode {
    Command,
    Unfocused
}

struct TuiViewModel {
    mode: TuiMode,
    app: Arc<Mutex<App>>,
}

impl TuiViewModel {
    fn new(app: Arc<Mutex<App>>) -> Self {
        Self {app, mode: TuiMode::Unfocused}
    }

    fn change_mode(&mut self, mode: TuiMode) {
        self.mode = mode;
    }

    fn change_waveform(&self, waveform: Waveform) {
        self.app.lock().unwrap().oscillator.set_waveform(waveform);
    }
}

#[derive(Copy, Clone)]
enum BorderKind {
    Single,
    Double,
    Heavy,
}
impl BorderKind {
    pub fn get_symbols(&self) -> [&str; 6] {
        match self {
            BorderKind::Heavy =>  ["┃", "━", "┏", "┓", "┗", "┛"],
            BorderKind::Single =>  ["│", "─", "┌", "┐", "└", "┘"],
            BorderKind::Double =>  ["║", "═", "╔", "╗", "╚", "╝"]
        }
    }
}

struct TuiRect {
    pub kind: BorderKind,
    from: (u16, u16),
    to: (u16, u16),
}
impl TuiRect {

    pub fn from_coords(kind: BorderKind, from: (u16, u16), to: (u16, u16)) -> Self {
        Self {
            kind,
            from,
            to,
        }
    }

    pub fn from_size(kind: BorderKind, pos: (u16, u16), size: (u16, u16)) -> Self {
        Self {
            kind,
            from: pos,
            to: (pos.0 + size.0, pos.1 + size.1)
        }
    }

    pub fn draw(&self) -> Result<()> {
        Self::draw_rect(self.kind, self.from, self.to)?;
        Ok(())
    }

    pub fn set_pos(&mut self, pos: (u16, u16)) {
        let sw = self.to.0 - self.from.0;
        let sh = self.to.1 - self.from.1;
        self.from = pos;
        self.to = (pos.0 + sw, pos.1 + sh);
    }

    pub fn set_size(&mut self, size: (u16, u16)) {
        self.to.0 = self.from.0 + size.0;
        self.to.1 = self.from.1 + size.1;
    }

    fn draw_rect(kind: BorderKind, from: (u16, u16), to: (u16,u16)) -> Result<()> {
        let mut stdo = stdout();
        let (pipe, dash, tlc, trc, blc, brc) = kind.get_symbols().into();

        let w_repeat = from.0.abs_diff(to.0).max(2) - 2;
        let h_repeat = from.1.abs_diff(to.1).max(2) - 2;

        Self::draw_line(w_repeat, from.0, from.1, tlc, dash, trc)?;
        for i in 1..= h_repeat {
            Self::draw_line(w_repeat, from.0, from.1+i, pipe, " ", pipe)?;
        }
        Self::draw_line(w_repeat, from.0, to.1.max(1)-1, blc, dash, brc)?;
        stdo.flush()?;
        Ok(())
    }

    fn draw_line(n: u16, x: u16, y: u16, cs: &str, cm: &str, ce: &str) -> Result<()> {
        stdout()
            .queue(cursor::MoveTo(x, y))?
            .queue(style::Print(cs))?
            .queue(style::Print(cm.repeat(n as usize)))?
            .queue(style::Print(ce))?
        ;
        Ok(())
    }
}

pub fn tui(app: Arc<Mutex<App>>) -> Result<()> {
    let mut viewmodel = TuiViewModel::new(app);

    let mut rect = TuiRect::from_size(BorderKind::Double, (5,5), (10, 5));
    let (mut w, mut h) = terminal::size()?;
    startup()?;
    loop {
        rect.set_pos((2,1));
        rect.set_size((w-4, h-2));
        rect.draw()?;
        if event::poll(Duration::from_millis(15))? {
            match event::read()? {
                Event::Resize(wi,he) => {
                    (w,h) = (wi, he);
                    stdout().execute(Clear(ClearType::All))?;
                },
                Event::Key(event) => match event.code {
                    KeyCode::Char('q') => break,
                    KeyCode::Char(':') => viewmodel.change_mode(TuiMode::Command),
                    KeyCode::Down => viewmodel.change_waveform(Waveform::Square),
                    KeyCode::Up => viewmodel.change_waveform(Waveform::Sine),
                    _ => {}
                },
                _ => {}
            }
        }
    }
    shutdown()?;
    Ok(())
}

fn startup() -> Result<()> {
    let (w,h) = terminal::size()?;
    let out = "Hello world!";
    enable_raw_mode()?;
    stdout()
        .queue(cursor::Hide)?
        .queue(Clear(ClearType::All))?
        .queue(cursor::MoveTo(w/2-(out.len()/2) as u16,h/2))?
        .queue(style::Print(out))?
        .flush()?
        ;
    Ok(())
}

fn shutdown() -> Result<()> {
    disable_raw_mode()?;
    stdout()
        .queue(cursor::Show)?
        .queue(cursor::SetCursorStyle::DefaultUserShape)?
        .queue(Clear(ClearType::All))?
        .queue(cursor::MoveTo(0,0))?
        .flush()?
        ;
    Ok(())
}

fn menu_screen() {
    // let mut stdout = stdout();
    // let (w,h) = terminal::size().unwrap();
}

fn home_screen() {

}

fn command_bar() {

}



fn thing() -> [&'static str; 5] {
    let letter = |c: char| {
        match c {
            'f' => [
                "   ████\n",
                "  ██   \n",
                "███████\n",
                "  ██   \n",
                "  ██   \n" ],
            'M' => [
                " ███  ███ \n",
                "██  ██  ██\n",
                "██  ██  ██\n",
                "██  ██  ██\n",
                "██  ██  ██\n"],
            'A' => [
                " ██████ \n",
                "██    ██\n",
                "████████\n",
                "██    ██\n",
                "██    ██\n"],
            'N' => [
                "███     ██\n",
                "██ ██   ██\n",
                "██  ██  ██\n",
                "██   ██ ██\n",
                "██     ███\n"],
            'G' => [
                " ██████ \n",
                "██      \n",
                "██  ████\n",
                "██    ██\n",
                " ██████ \n"],
            'R' => [
                "███████ \n",
                "██    ██\n",
                "███████ \n",
                "██  ██  \n",
                "██    ██\n"],
            'O' => [
                "████████\n",
                "██    ██\n",
                "██    ██\n",
                "██    ██\n",
                "████████\n"],
            'V' => [
                "██     ██\n",
                "██     ██\n",
                " ██   ██ \n",
                "  ██ ██  \n",
                "   ███   \n"],
            'E' => [
                "███████\n",
                "██     \n",
                "███████\n",
                "██     \n",
                "███████\n"],
            _ => [
                "██    ██\n",
                "██    ██\n",
                "██    ██\n",
                "██    ██\n",
                "████████\n"]

        }
    };
    letter('f')
}