use std::io::{stdout, Write, Result};
use crossterm::{cursor, QueueableCommand, style};

#[derive(Copy, Clone)]
pub enum BorderKind {
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

pub struct TuiRect {
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
        let (pipe, dash, tlc, trc, blc, brc) = kind.get_symbols().into();

        let w_repeat = from.0.abs_diff(to.0) - 2;
        let h_repeat = from.1.abs_diff(to.1) - 2;

        Self::draw_line(w_repeat, from.0, from.1, tlc, dash, trc)?;
        for i in 1..= h_repeat {
            Self::draw_line(w_repeat, from.0, from.1+i, pipe, " ", pipe)?;
        }
        Self::draw_line(w_repeat, from.0, to.1, blc, dash, brc)?;

        Ok(())
    }

    fn draw_line(n: u16, x: u16, y: u16, cs: &str, cm: &str, ce: &str) -> Result<()> {
        stdout()
            .queue(cursor::MoveTo(x, y))?
            .queue(style::Print(cs))?
            .queue(style::Print(cm.repeat(n as usize)))?
            .queue(style::Print(ce))?
            .flush()?
        ;
        Ok(())
    }
}
