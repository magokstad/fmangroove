use crate::app::{App, Instruction};
use crate::instrument::oscillator::Waveform;
use crate::view::tui_elements::TuiSplit;
use crate::view::tui_elements::{TuiStructure, TuiStructureLink, TuiTiles};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyModifiers};
use crossterm::terminal::{self, disable_raw_mode, enable_raw_mode, Clear, ClearType};
use crossterm::{cursor, style, QueueableCommand};
use std::io::{stdout, Result, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;

mod tui_elements;

enum LoopStatus {
    Continue,
    Break,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum TuiMode {
    Command,
    Unfocused,
}

struct TuiViewModel {
    mode: TuiMode,
    app: Arc<Mutex<App>>,
    cmd_buf: String,
    status_buf: String,
}

impl TuiViewModel {
    fn new(app: Arc<Mutex<App>>) -> Self {
        Self {
            app,
            mode: TuiMode::Unfocused,
            cmd_buf: String::new(),
            status_buf: String::new(),
        }
    }

    fn change_mode(&mut self, mode: TuiMode) {
        self.mode = mode;
        if mode == TuiMode::Unfocused {
            self.cmd_buf.clear()
        }
    }

    fn test_apply_instruction(&mut self, i: usize, instruction: Instruction) {
        if let Some(instrument) = self.app.lock().unwrap().instruments.get_mut(i) {
            instrument.apply_instruction(instruction)
        }
    }
}

pub fn tui(app: Arc<Mutex<App>>) -> Result<()> {
    let mut viewmodel = TuiViewModel::new(app);

    let tiles = TuiTiles {
        structure: TuiStructure {
            kind: TuiSplit::VSplit,
            stuffs: vec![
                TuiStructureLink::Element(String::from("One")),
                TuiStructureLink::Element(String::from("Two")),
                TuiStructureLink::Structure(TuiStructure {
                    kind: TuiSplit::VSplit,
                    stuffs: vec![
                        TuiStructureLink::Structure(TuiStructure {
                            kind: TuiSplit::HSplit,
                            stuffs: vec![
                                TuiStructureLink::Element(String::from("Three")),
                                TuiStructureLink::Element(String::from("Four")),
                                TuiStructureLink::Element(String::from("Five")),
                            ],
                        }),
                        TuiStructureLink::Element(String::from("Six")),
                    ],
                }),
            ],
        },
    };
    let (mut w, mut h) = terminal::size()?;
    startup()?;
    loop {
        tiles.draw()?;
        command_bar(&viewmodel)?;
        stdout().flush()?;
        if event::poll(Duration::from_millis(15))? {
            match event::read()? {
                Event::Resize(wi, he) => {
                    (w, h) = (wi, he);
                    stdout().queue(Clear(ClearType::All))?;
                }
                Event::Key(event) => match viewmodel.mode {
                    TuiMode::Unfocused => match event.code {
                        KeyCode::Char(':') => viewmodel.change_mode(TuiMode::Command),
                        KeyCode::Char('c') | KeyCode::Char('d') => {
                            if event.modifiers == KeyModifiers::CONTROL {
                                break;
                            }
                        }
                        KeyCode::Esc => viewmodel.change_mode(TuiMode::Unfocused),
                        KeyCode::Down => viewmodel
                            .test_apply_instruction(0, Instruction::Waveform(Waveform::Square)),
                        KeyCode::Up => viewmodel
                            .test_apply_instruction(0, Instruction::Waveform(Waveform::Sine)),
                        _ => {}
                    },
                    TuiMode::Command => match handle_command(&mut viewmodel, event) {
                        LoopStatus::Break => break,
                        LoopStatus::Continue => {},
                    }
                },
                _ => {}
            }
        }
    }
    shutdown()?;
    Ok(())
}

// TODO: Terrible parser, improve
fn handle_command(viewmodel: &mut TuiViewModel, event: KeyEvent) -> LoopStatus {
    match event.code {
        KeyCode::Esc => viewmodel.change_mode(TuiMode::Unfocused),
        KeyCode::Char(c) => {
            viewmodel.cmd_buf.push(c);
        }
        KeyCode::Backspace => {
            viewmodel.cmd_buf.pop();
        }
        KeyCode::Enter => {
            viewmodel.status_buf.clear();
            let stuff: Vec<&str> = viewmodel.cmd_buf.split(" ").collect();
            let none = "";
            let command = *stuff.get(0).unwrap_or(&none);
            match command {
                "quit" | "q" => return LoopStatus::Break,
                "on" | "off" => {
                    if let Some(arg1) = stuff.get(1) {
                        if let Ok(osc) = arg1.parse::<usize>() {
                            viewmodel.test_apply_instruction(osc, Instruction::SetState(command.eq("on")));
                        }
                    }
                }
                "vibon" | "viboff" => {
                    if let Some(arg1) = stuff.get(1) {
                        if let Ok(osc) = arg1.parse::<usize>() {
                            viewmodel.test_apply_instruction(osc, Instruction::SetVibrato(command.eq("vibon")));
                        }
                    }
                }
                "hz" => {
                    if let (Some(arg1), Some(arg2)) = (stuff.get(1), stuff.get(2)) {
                        if let (Ok(osc), Ok(hz)) =
                            (arg1.parse::<usize>(), arg2.parse::<f32>())
                        {
                            viewmodel.test_apply_instruction(
                                osc,
                                Instruction::Frequency(hz),
                            )
                        }
                    }
                }
                _ =>  viewmodel.status_buf = String::from(format!("unknown command '{}'", viewmodel.cmd_buf))
            }
            viewmodel.change_mode(TuiMode::Unfocused)
        }
        _ => {}
    }

    LoopStatus::Continue
}

fn startup() -> Result<()> {
    enable_raw_mode()?;
    stdout()
        .queue(cursor::Hide)?
        .queue(Clear(ClearType::All))?
        .flush()?;
    Ok(())
}

fn shutdown() -> Result<()> {
    disable_raw_mode()?;
    stdout()
        .queue(cursor::Show)?
        .queue(cursor::SetCursorStyle::DefaultUserShape)?
        .queue(Clear(ClearType::Purge))?
        .queue(cursor::MoveTo(0, 0))?
        .flush()?;
    Ok(())
}

fn menu_screen() {
    // let mut stdout = stdout();
    // let (w,h) = terminal::size().unwrap();
}

fn home_screen() {}

fn command_bar(vm: &TuiViewModel) -> Result<()> {
    let (w, h) = terminal::size()?;
    stdout()
        .queue(cursor::MoveTo(0, h - 1))?
        .queue(style::Print(" ".repeat(w as usize)))?
        .queue(cursor::MoveTo(0, h - 1))?
        .queue(style::Print(
            if vm.mode == TuiMode::Command { String::from(":") + vm.cmd_buf.as_str() } else { vm.status_buf.clone() }))?
    // .flush()?
    ;
    Ok(())
}

fn thing() -> [&'static str; 5] {
    let letter = |c: char| match c {
        'f' => [
            "   ████",
            "  ██   ",
            "███████",
            "  ██   ",
            "  ██   ",
        ],
        'M' => [
            " ███  ███ ",
            "██  ██  ██",
            "██  ██  ██",
            "██  ██  ██",
            "██  ██  ██",
        ],
        'A' => [
            " ██████ ",
            "██    ██",
            "████████",
            "██    ██",
            "██    ██",
        ],
        'N' => [
            "███     ██",
            "██ ██   ██",
            "██  ██  ██",
            "██   ██ ██",
            "██     ███",
        ],
        'G' => [
            " ██████ ",
            "██      ",
            "██  ████",
            "██    ██",
            " ██████ ",
        ],
        'R' => [
            "███████ ",
            "██    ██",
            "███████ ",
            "██  ██  ",
            "██    ██",
        ],
        'O' => [
            "████████",
            "██    ██",
            "██    ██",
            "██    ██",
            "████████",
        ],
        'V' => [
            "██     ██",
            "██     ██",
            " ██   ██ ",
            "  ██ ██  ",
            "   ███   ",
        ],
        'E' => [
            "███████",
            "██     ",
            "███████",
            "██     ",
            "███████",
        ],
        _ => [
            "",
            "",
            "",
            "",
            "",
        ],
    };
    letter('f')
}
