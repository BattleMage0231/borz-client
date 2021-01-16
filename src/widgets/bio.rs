use crossterm::event::{KeyCode, KeyEvent};
use std::cmp::min;
use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::symbols::Marker;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Widget, Wrap};

#[derive(Debug, Clone)]
pub struct BioWidget {
    content: String,
    selected_row: usize,
    focused: bool,
}

impl BioWidget {
    pub fn new(content: String) -> BioWidget {
        BioWidget {
            content,
            selected_row: 0,
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

impl Widget for BioWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let paragraph = Paragraph::new(self.content)
            .block(
                Block::default()
                    .title("Subgroups")
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
