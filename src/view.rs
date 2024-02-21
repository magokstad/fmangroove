use crate::app::App;
use crate::view::tui_elements::TuiSplit;
use crate::view::tui_elements::{TuiStructure, TuiStructureLink, TuiTiles};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use crossterm::terminal::{self, Clear, ClearType, disable_raw_mode, enable_raw_mode};
use crossterm::{cursor, QueueableCommand, style};
use std::io::{stdout, Write};
use std::sync::{Arc, Mutex};
use std::time::Duration;
use crate::instruction::InstructionKind;

mod tui_elements;
mod grid_select;

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
    tiles: TuiTiles,
    target_instrument: Option<u128>,
    target_tick: Option<u128>,
    cmd_buf: String,
    status_buf: String,
}

impl TuiViewModel {
    fn new(app: Arc<Mutex<App>>) -> Self {
        Self {
            app,
            mode: TuiMode::Unfocused,
            tiles: TuiTiles {
                structure: TuiStructure {
                    kind: TuiSplit::HSplit,
                    stuffs: vec![
                        TuiStructureLink::Element("Testing".to_string()),
                        TuiStructureLink::Element("Testingsssssss2".to_string()),
                        TuiStructureLink::Element("Testingsssssss3".to_string()),
                        TuiStructureLink::Structure(TuiStructure {
                            kind: TuiSplit::VSplit,
                            stuffs: vec![
                                TuiStructureLink::Element("Testing".to_string()),
                                TuiStructureLink::Element("Testingsssssss2".to_string()),
                            ]
                        })
                    ]
                }
            },
            cmd_buf: String::new(),
            status_buf: String::new(),
            target_tick: None,
            target_instrument: None
        }
    }

    fn change_mode(&mut self, mode: TuiMode) {
        self.mode = mode;
        if mode == TuiMode::Unfocused {
            self.cmd_buf.clear()
        }
    }

    fn draw(&mut self) -> std::io::Result<()> {
        self.tiles.draw()?;
        Ok(())
    }

    fn play(&mut self) {
       self.app.lock().unwrap().play();
    }

    fn pause(&mut self) {
        self.app.lock().unwrap().pause();
    }

    fn reset(&mut self) {
        self.app.lock().unwrap().reset();
    }

    fn add_instruction(&mut self, kind: InstructionKind) -> Result<(), String> {
        if self.target_instrument.is_none() {
            return Err(String::from("No target instrument set"))
        }
        if self.target_tick.is_none() {
            return Err(String::from("No target tick set"))
        }
        // let inst = Instruction::new(kind, self.target_instrument.unwrap());
        // self.app.lock().unwrap().instructions.asd();
        self.app.lock().unwrap().instructions.insert(self.target_instrument.unwrap(), self.target_tick.unwrap(), kind);
        Ok(())
    }
}

pub fn tui(app: Arc<Mutex<App>>) -> std::io::Result<()> {
    let mut viewmodel = TuiViewModel::new(app);

    startup()?;
    event_loop(viewmodel)?;
    shutdown()?;
    Ok(())
}

fn event_loop(mut viewmodel: TuiViewModel) -> std::io::Result<()> {
    loop {
        viewmodel.draw()?;
        command_bar(&viewmodel)?;
        stdout().flush()?;
        if event::poll(Duration::from_millis(8))? {
            match event::read()? {
                Event::Resize(_,_) => {
                    stdout()
                        .queue(Clear(ClearType::All))?
                        .queue(Clear(ClearType::Purge))?
                    ;
                },
                Event::Key(event) => match viewmodel.mode {
                    TuiMode::Unfocused => match event.code {
                        KeyCode::Char(':') => viewmodel.change_mode(TuiMode::Command),
                        KeyCode::Char('c') | KeyCode::Char('d') => {
                            if event.modifiers == KeyModifiers::CONTROL {
                                break;
                            }
                        }
                        KeyCode::Esc => viewmodel.change_mode(TuiMode::Unfocused),
                        _ => {}
                    },
                    // TODO: this code looks confusing, consider handling breaks another way?
                    TuiMode::Command => if let LoopStatus::Break = handle_command(&mut viewmodel, event)? { break; }
                },
                _ => {}
            }
        }
    }
    Ok(())
}

// TODO: Terrible parser, improve
fn handle_command(viewmodel: &mut TuiViewModel, event: KeyEvent) -> std::io::Result<LoopStatus> {
    if let KeyEventKind::Release = event.kind {
        return Ok(LoopStatus::Continue)
    }
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
            let command = *stuff.get(0).unwrap_or(&"");
            match command {
                "quit" | "q" => return Ok(LoopStatus::Break),
                "clear" | "cls" => {
                    stdout()
                        .queue(Clear(ClearType::All))?
                        .queue(Clear(ClearType::Purge))?;
                },
                "play" => viewmodel.play(),
                "pause" | "stop" => viewmodel.pause(),
                "reset" => viewmodel.reset(),
                "time" => if let Ok(f) = stuff.get(1).unwrap_or(&"").parse::<f32>() {
                    viewmodel.target_tick = Some((f * viewmodel.app.lock().unwrap().get_sample_rate()) as u128);
                },
                "inst" => if let Ok(u) = stuff.get(1).unwrap_or(&"").parse::<u128>() {
                    viewmodel.target_instrument = Some(u);
                },
                _ => match InstructionKind::parse(viewmodel.cmd_buf.clone()) {
                    Ok(inst) => if let Err(msg) = viewmodel.add_instruction(inst) { viewmodel.status_buf = msg; }
                    Err(msg) => viewmodel.status_buf = msg
                }
            }
            viewmodel.change_mode(TuiMode::Unfocused)
        }
        _ => {}
    }

    Ok(LoopStatus::Continue)
}

fn startup() -> std::io::Result<()> {
    enable_raw_mode()?;
    stdout()
        .queue(cursor::Hide)?
        .queue(Clear(ClearType::All))?
        .flush()?;
    Ok(())
}

fn shutdown() -> std::io::Result<()> {
    disable_raw_mode()?;
    stdout()
        .queue(cursor::Show)?
        .queue(cursor::SetCursorStyle::DefaultUserShape)?
        .queue(Clear(ClearType::Purge))?
        .queue(Clear(ClearType::All))?
        .queue(cursor::MoveTo(0, 0))?
        .flush()?;
    Ok(())
}

fn menu_screen() {
    // let mut stdout = stdout();
    // let (w,h) = terminal::size().unwrap();
}

fn home_screen() {}

fn command_bar(vm: &TuiViewModel) -> std::io::Result<()> {
    let (w, h) = terminal::size()?;
    stdout()
        .queue(cursor::MoveTo(0, h - 1))?
        .queue(style::Print(" ".repeat(w as usize)))?
        .queue(cursor::MoveTo(0, h - 1))?
        .queue(style::Print(if vm.mode == TuiMode::Command { String::from(":") + vm.cmd_buf.as_str() } else { vm.status_buf.clone() }))?
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
