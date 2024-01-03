use crossterm::style::Attribute::Bold;
use crossterm::style::Stylize;
use crossterm::{cursor, style, QueueableCommand};
use std::io;
use std::io::{stdout, Result, Write};

pub struct TuiTiles {
    pub structure: TuiStructure,
}

impl TuiTiles {
    pub fn draw(&self) -> Result<()> {
        let (w, h) = crossterm::terminal::size()?;
        if w < 30 || h < 15 {
            stdout()
                .queue(cursor::MoveTo(0, 0))?
                .queue(style::Print("Terminal too small! Please resize"))?
            // .flush()?
            ;
            return Ok(());
        }
        self.structure.draw(1, w, 1, h - 2)?;
        Ok(())
    }
}

pub enum TuiSplit {
    VSplit,
    HSplit,
}

pub enum TuiStructureLink {
    Structure(TuiStructure),
    Element(String),
    Empty,
}

pub struct TuiStructure {
    pub kind: TuiSplit,
    pub stuffs: Vec<TuiStructureLink>,
}

pub struct TuiPanel {

}

impl TuiStructure {
    fn draw(&self, left: u16, right: u16, top: u16, bottom: u16) -> Result<()> {
        let splits = self.stuffs.len().max(1);

        let (mut new_top, mut new_bottom, mut new_left, mut new_right) = (top, bottom, left, right);

        for (ind, structure) in self.stuffs.iter().enumerate() {
            match self.kind {
                TuiSplit::HSplit => {
                    let h_interval = (bottom - top) / (splits as u16);
                    new_top = top + h_interval * (ind as u16);
                    new_bottom = top + h_interval * (ind as u16 + 1);
                }
                TuiSplit::VSplit => {
                    let v_interval = (right - left) / (splits as u16);
                    new_left = left + v_interval * (ind as u16);
                    new_right = left + v_interval * (ind as u16 + 1);
                }
            }
            match structure {
                TuiStructureLink::Structure(s) => {
                    s.draw(new_left, new_right, new_top, new_bottom)?
                }
                TuiStructureLink::Element(n) => TuiRect::draw_rect(
                    String::from(n),
                    BorderKind::Single,
                    (new_left, new_top),
                    (new_right, new_bottom),
                )?,
                TuiStructureLink::Empty => {}
            }
        }
        Ok(())
    }
}

#[derive(Copy, Clone)]
pub enum BorderKind {
    Single,
    Double,
    Heavy,
}

impl BorderKind {
    pub fn get_symbols(&self) -> [&str; 6] {
        match self {
            BorderKind::Heavy => ["┃", "━", "┏", "┓", "┗", "┛"],
            BorderKind::Single => ["│", "─", "┌", "┐", "└", "┘"],
            BorderKind::Double => ["║", "═", "╔", "╗", "╚", "╝"],
        }
    }
}

pub struct TuiRect;

impl TuiRect {
    pub fn draw_rect(
        name: String,
        kind: BorderKind,
        from: (u16, u16),
        to: (u16, u16),
    ) -> Result<()> {
        let (pipe, dash, tlc, trc, blc, brc) = kind.get_symbols().into();

        let w_repeat = from.0.abs_diff(to.0);
        let h_repeat = from.1.abs_diff(to.1);

        let tag = String::from(" ")
            + name
                .chars()
                .take((w_repeat as usize - 2).max(0))
                .collect::<String>()
                .as_str()
            + " ";

        Self::draw_line(w_repeat, from.0, from.1, tlc, dash, trc)?;
        for i in 1..h_repeat {
            Self::draw_line(w_repeat, from.0, from.1 + i, pipe, " ", pipe)?;
        }
        Self::draw_line(w_repeat, from.0, to.1, blc, dash, brc)?;
        stdout()
            .queue(cursor::MoveTo(from.0 + 1, from.1))?
            .queue(style::PrintStyledContent(tag.attribute(Bold)))?
        // .flush()?
        ;

        Ok(())
    }

    fn draw_line(n: u16, x: u16, y: u16, cs: &str, cm: &str, ce: &str) -> Result<()> {
        stdout()
            .queue(cursor::MoveTo(x, y))?
            .queue(style::Print(cs))?
            .queue(style::Print(cm.repeat((n - 2).max(0) as usize)))?
            .queue(style::Print(ce))?
        // .flush()?
        ;
        Ok(())
    }
}
