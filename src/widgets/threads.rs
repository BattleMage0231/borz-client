use crossterm::event::{KeyCode, KeyEvent};
use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::symbols::Marker;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Widget, Wrap};
use std::cmp::min;

const UP_ARROW: char = 38_u8 as char;
const DOWN_ARROW: char = 40_u8 as char;

#[derive(Debug, Clone)]
pub struct ThreadsWidget {
    threads: Vec<String>,
    authors: Vec<String>,
    selected_row: usize,
    focused: bool,
    on_left: bool,
    scroll_top: usize,
    scroll_bottom: usize,
}

impl ThreadsWidget {
    pub fn new(threads: Vec<String>, authors: Vec<String>) -> ThreadsWidget {
        ThreadsWidget {
            threads,
            authors,
            selected_row: 0,
            focused: false,
            on_left: true,
            scroll_top: 0,
            scroll_bottom: 0,
        }
    }

    pub fn focus(&mut self) {
        self.focused = true;
        self.selected_row = 0;
    }

    pub fn unfocus(&mut self) {
        self.focused = false;
    }

    pub fn resize(&mut self, area: &Rect) {
        if area.height as usize - 2 == self.scroll_bottom - self.scroll_top {
            return;
        }
        self.scroll_top = 0;
        self.scroll_bottom = min(self.threads.len(), area.height as usize - 2);
        self.scroll();
    }

    pub fn scroll(&mut self) {
        if self.scroll_top > self.selected_row {
            let amt = self.scroll_top - self.selected_row;
            self.scroll_bottom -= amt;
            self.scroll_top -= amt;
        } else if self.scroll_bottom <= self.selected_row {
            let amt = self.selected_row - self.scroll_bottom + 1;
            self.scroll_top += amt;
            self.scroll_bottom += amt;
        }
    }

    pub fn update(&mut self, key: KeyEvent) {
        if !self.focused {
            return;
        }
        if key.modifiers.is_empty() {
            match key.code {
                KeyCode::Down if self.selected_row < self.threads.len() - 1 => {
                    self.selected_row += 1;
                    self.scroll();
                },
                KeyCode::Up if self.selected_row > 0 => {
                    self.selected_row -= 1;
                    self.scroll();
                },
                KeyCode::Left if !self.on_left => {
                    self.on_left = true;
                },
                KeyCode::Right if self.on_left => {
                    self.on_left = false;
                }
                _ => {},
            }
        }
    }
}

impl Widget for ThreadsWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(85), Constraint::Percentage(15)].as_ref())
            .split(area);
        let mut text = vec![];
        for (pos, thread) in self.threads[self.scroll_top..self.scroll_bottom].iter().enumerate() {
            let span = Spans::from(vec![Span::styled({
                if thread.len() > area.width as usize {
                    String::from(&thread[..(chunks[0].width as usize - 5)]) + "..."
                } else {
                    thread.clone()
                }
            }, {
                if self.focused && pos + self.scroll_top == self.selected_row && self.on_left {
                    Style::default().bg(Color::Red)
                } else {
                    Style::default()
                }
            })]);
            text.push(span);
        }
        let paragraph = Paragraph::new(text)
            .block(Block::default().title("Threads").borders(Borders::ALL))
            .style(Style::default().bg(Color::Green).fg(if self.focused {
                Color::Cyan
            } else {
                Color::White
            }))
            .wrap(Wrap { trim: true });
        paragraph.render(chunks[0], buf);
        let mut text = vec![];
        for (pos, author) in self.authors[self.scroll_top..self.scroll_bottom].iter().enumerate() {
            let span = Spans::from(vec![Span::styled({
                if author.len() > area.width as usize {
                    String::from(&author[..(chunks[1].width as usize - 5)]) + "..."
                } else {
                    author.clone()
                }
            }, {
                if self.focused && pos + self.scroll_top == self.selected_row && !self.on_left {
                    Style::default().bg(Color::Red)
                } else {
                    Style::default()
                }
            })]);
            text.push(span);
        }
        let paragraph = Paragraph::new(text)
            .block(
                Block::default()
                    .title("Authors")
                    .borders(Borders::ALL)
                    .style(Style::default().bg(Color::Green).fg(if self.focused {
                        Color::Cyan
                    } else {
                        Color::White
                    })),
            )
            .wrap(Wrap { trim: true });
        paragraph.render(chunks[1], buf);
    }
}
