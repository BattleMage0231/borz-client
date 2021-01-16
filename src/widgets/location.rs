use crossterm::event::KeyEvent;
use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::symbols::Marker;
use tui::widgets::{Block, Borders, Paragraph, Widget, Wrap};

#[derive(Debug, Clone)]
pub struct LocationWidget {
    loc: String,
    focused: bool,
}

impl LocationWidget {
    pub fn new(loc: String) -> LocationWidget {
        LocationWidget {
            loc,
            focused: false,
        }
    }

    pub fn focus(&mut self) {
        self.focused = true;
    }

    pub fn unfocus(&mut self) {
        self.focused = false;
    }

    pub fn update(&mut self, key: KeyEvent) {}
}

impl Widget for LocationWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let paragraph = Paragraph::new(self.loc)
            .block(
                Block::default()
                    .title("Location")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Green).fg(if self.focused {
                        Color::Cyan
                    } else {
                        Color::White
                    })),
            )
            .wrap(Wrap { trim: true });
        paragraph.render(area, buf);
    }
}
