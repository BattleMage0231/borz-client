use crate::app::{App, AppPage};
use crate::widgets::page::ThreadPage;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::cmp::min;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Widget, Wrap};

#[derive(Debug, Clone)]
pub struct MessagesWidget {
    author: String,
    authors: Vec<String>,
    messages: Vec<Vec<String>>,
    input_buffer: Vec<Vec<char>>,
    selected_message: usize,
    selected_row: usize,
    focused: bool,
    editing: bool,
    scroll_top: usize,
    scroll_bottom: usize,
    new: bool,
}

impl MessagesWidget {
    pub fn new(
        authors: Vec<String>,
        messages: Vec<Vec<String>>,
        author: String,
        new: bool,
    ) -> MessagesWidget {
        MessagesWidget {
            author,
            authors,
            messages,
            input_buffer: Vec::new(),
            selected_message: 0,
            selected_row: 0,
            focused: false,
            editing: new,
            scroll_top: 0,
            scroll_bottom: 0,
            new,
        }
    }

    pub fn focus(&mut self) {
        self.focused = true;
    }

    pub fn unfocus(&mut self) {
        self.focused = false;
    }

    pub fn resize(&mut self, area: &Rect) {
        if area.height as usize - 4 != self.scroll_bottom - self.scroll_top {
            self.scroll_top = 0;
            self.scroll_bottom = min(
                {
                    if self.editing {
                        self.input_buffer.len()
                    } else {
                        self.messages[self.selected_message].len()
                    }
                },
                area.height as usize - 4,
            );
        }
        if self.input_buffer.is_empty() {
            self.input_buffer.push(Vec::new());
        }
        self.scroll();
        if self.editing {
            for index in self.scroll_top..self.scroll_bottom {
                self.input_buffer[index].truncate(area.width as usize - 2);
            }
        }
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

    pub fn update(&mut self, key: KeyEvent) -> Box<dyn for<'a> Fn(&'a mut App)> {
        if !self.focused {
            return Box::new(|_| {});
        }
        if key.modifiers.is_empty() {
            match key.code {
                KeyCode::Down => {
                    if self.editing {
                        if self.input_buffer.is_empty() {
                            self.input_buffer.push(Vec::new());
                        }
                        if self.selected_row < self.input_buffer.len() - 1 {
                            self.selected_row += 1;
                        }
                    } else {
                        if self.selected_row < self.messages[self.selected_message].len() - 1 {
                            self.selected_row += 1;
                        }
                    }
                    self.scroll();
                }
                KeyCode::Up => {
                    if self.editing {
                        if self.input_buffer.is_empty() {
                            self.input_buffer.push(Vec::new());
                        }
                        if self.selected_row > 0 {
                            self.selected_row -= 1;
                        }
                    } else {
                        if self.selected_row > 0 {
                            self.selected_row -= 1;
                        }
                    }
                    self.scroll();
                }
                KeyCode::Left => {
                    if self.new {
                        return Box::new(|_| {});
                    }
                    if self.editing {
                        if self.input_buffer.is_empty() {
                            self.input_buffer.push(Vec::new());
                        }
                        self.editing = false;
                        self.selected_message = self.messages.len() - 1;
                        self.selected_row = 0;
                        self.scroll_top = 0;
                        self.scroll_bottom = 0;
                    } else {
                        if self.selected_message > 0 {
                            self.selected_message -= 1;
                            self.selected_row = 0;
                            self.scroll_top = 0;
                            self.scroll_bottom = 0;
                        }
                    }
                }
                KeyCode::Right => {
                    if !self.editing {
                        if self.selected_message == self.messages.len() - 1 {
                            self.editing = true;
                            self.selected_row = 0;
                            self.scroll_top = 0;
                            self.scroll_bottom = 0;
                        } else if self.selected_message < self.messages.len() - 1 {
                            self.selected_message += 1;
                            self.selected_row = 0;
                            self.scroll_top = 0;
                            self.scroll_bottom = 0;
                        }
                    }
                }
                KeyCode::Enter => {
                    if self.editing {
                        if self.input_buffer.is_empty() {
                            self.input_buffer.push(Vec::new());
                        }
                        if self.selected_row == self.input_buffer.len() - 1 {
                            self.input_buffer.push(Vec::new());
                            self.selected_row += 1;
                            self.scroll();
                        } else {
                            self.selected_row += 1;
                            self.input_buffer.insert(self.selected_row, Vec::new());
                            self.scroll_bottom -= 1;
                            self.scroll();
                        }
                    }
                }
                KeyCode::Backspace => {
                    if self.editing {
                        if self.input_buffer.is_empty() {
                            self.input_buffer.push(Vec::new());
                        }
                        self.input_buffer.remove(self.selected_row);
                        if self.input_buffer.is_empty() {
                            self.input_buffer.push(Vec::new());
                        }
                        if self.selected_row >= self.input_buffer.len() - 1 && self.selected_row > 0
                        {
                            self.selected_row -= 1;
                        }
                        self.scroll_bottom = min(self.scroll_bottom, self.input_buffer.len());
                    }
                }
                KeyCode::Insert => {
                    let mut content = String::new();
                    for line in self.input_buffer.iter() {
                        for chr in line.iter() {
                            content.push(*chr);
                        }
                        content.push('\n');
                    }
                    content = String::from(content.trim());
                    if !content.is_empty() {
                        return Box::new(move |app| {
                            if let AppPage::Thread(tp) = app.get_page().unwrap() {
                                tp.fetcher
                                    .mutate_thread_reply(tp.thread_id.clone(), content.clone());
                                let tp = tp.clone();
                                // reset page
                                app.pop_page();
                                app.push_page(AppPage::Thread(ThreadPage::new(
                                    tp.fetcher.clone(),
                                    tp.group_path.clone(),
                                    tp.thread_id.clone(),
                                    tp.username.clone(),
                                    false,
                                )));
                            } else {
                                panic!("Wrong page execution");
                            }
                        });
                    }
                }
                KeyCode::Char(c) => {
                    if self.editing {
                        if c.is_ascii_alphanumeric() || c.is_ascii_punctuation() || c == ' ' {
                            if self.input_buffer.is_empty() {
                                self.input_buffer.push(Vec::new());
                            }
                            self.input_buffer[self.selected_row].push(c);
                        }
                    }
                }
                _ => {}
            }
        } else if key.modifiers == KeyModifiers::SHIFT {
            match key.code {
                KeyCode::Char(c) => {
                    if self.editing {
                        if c.is_ascii_alphanumeric() || c.is_ascii_punctuation() || c == ' ' {
                            if self.input_buffer.is_empty() {
                                self.input_buffer.push(Vec::new());
                            }
                            self.input_buffer[self.selected_row].push(c);
                        }
                    }
                }
                _ => {}
            }
        }
        return Box::new(|_| {});
    }
}

impl Widget for MessagesWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut text = vec![];
        text.push(Spans::from(vec![
            Span::raw("Author: "),
            Span::styled(
                {
                    if self.editing {
                        self.author
                    } else {
                        self.authors[self.selected_message].clone()
                    }
                },
                Style::default().fg(Color::Blue),
            ),
        ]));
        text.push(Spans::from(vec![Span::raw("Message:")]));
        if !self.editing {
            for (pos, line) in self.messages[self.selected_message]
                [self.scroll_top..self.scroll_bottom]
                .iter()
                .enumerate()
            {
                let span = Spans::from(vec![Span::styled(line.clone(), {
                    if self.focused && pos + self.scroll_top == self.selected_row {
                        Style::default().bg(Color::Red)
                    } else {
                        Style::default()
                    }
                })]);
                text.push(span);
            }
        } else {
            for (pos, line) in self.input_buffer[self.scroll_top..self.scroll_bottom]
                .iter()
                .enumerate()
            {
                let s: String = line.iter().collect();
                let span = Spans::from(vec![Span::styled(s, {
                    if self.focused && pos + self.scroll_top == self.selected_row {
                        Style::default().bg(Color::Red)
                    } else {
                        Style::default()
                    }
                })]);
                text.push(span);
            }
        }
        let s = format!(
            "Message {} / {}",
            1 + self.selected_message,
            self.messages.len()
        );
        let paragraph = Paragraph::new(text)
            .block(
                Block::default()
                    .title({
                        if self.editing {
                            "Reply"
                        } else {
                            &s[..]
                        }
                    })
                    .borders(Borders::ALL),
            )
            .style(Style::default().bg(Color::Green).fg(if self.focused {
                Color::Cyan
            } else {
                Color::White
            }))
            .wrap(Wrap { trim: true });
        paragraph.render(area, buf);
    }
}
