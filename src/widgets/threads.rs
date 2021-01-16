use tui::buffer::Buffer;
use tui::symbols::Marker;
use tui::widgets::{Widget, Block, Borders, Paragraph, Wrap};
use tui::layout::{Layout, Constraint, Direction, Rect};
use crossterm::event::{KeyEvent, KeyCode};

const UP_ARROW: char = 38_u8 as char;
const DOWN_ARROW: char = 40_u8 as char;

#[derive(Debug, Clone)]
pub struct ThreadsWidget {
    threads: Vec<String>,
    authors: Vec<String>,
    selected_row: usize,
    focused: bool,
}

impl ThreadsWidget {
    pub fn new(threads: Vec<String>, authors: Vec<String>) -> ThreadsWidget {
        ThreadsWidget {
            threads,
            authors,
            selected_row: 0,
            focused: false
        }
    }

    pub fn focus(&mut self) {
        self.focused = true;
        self.selected_row = 0;
    }

    pub fn unfocus(&mut self) {
        self.focused = false;
    }

    pub fn update(&mut self, key: KeyEvent) {
        if !self.focused {
            return;
        }
        if key.modifiers.is_empty() {
            if let KeyCode::Char(c) = key.code {
                if c == UP_ARROW && self.selected_row > 0 {
                    self.selected_row -= 1;
                } else if c == DOWN_ARROW && self.selected_row < self.threads.len() - 1 {
                    self.selected_row += 1;
                }
            }
        }
    }
}

impl Widget for ThreadsWidget {
    fn render(mut self, area: Rect, buf: &mut Buffer) {
        let mut text = String::new();
        for (thread, author) in self.threads.iter().zip(self.authors.iter_mut()) {
            text.extend(thread.chars());
            text += &(" ".repeat(15))[..];
            text.extend(author.chars());
            text += "\n";
        }
        let paragraph = Paragraph::new(text)
            .block(Block::default().title("Threads").borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        paragraph.render(area, buf);
    }
}
