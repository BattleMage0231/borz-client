use crossterm::event::{KeyCode, KeyEvent};
use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::style::{Color, Style};
use tui::symbols::Marker;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Widget, Wrap};
use std::cmp::min;

#[derive(Debug, Clone)]
pub struct SubgroupsWidget {
    group: String,
    subgroups: Vec<String>,
    selected_row: usize,
    focused: bool,
    scroll_top: usize,
    scroll_bottom: usize,
}

impl SubgroupsWidget {
    pub fn new(group: String, subgroups: Vec<String>) -> SubgroupsWidget {
        SubgroupsWidget {
            group,
            subgroups,
            selected_row: 0,
            focused: false,
            scroll_top: 0,
            scroll_bottom: 0,
        }
    }

    pub fn focus(&mut self) {
        self.focused = true;
    }

    pub fn unfocus(&mut self) {
        self.focused = false;
    }

    pub fn resize(&mut self, area: &Rect) {
        if area.height as usize - 2 == self.scroll_bottom - self.scroll_top {
            return;
        }
        self.scroll_top = 0;
        self.scroll_bottom = min(self.subgroups.len(), area.height as usize - 2);
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
                KeyCode::Down if self.selected_row < self.subgroups.len() - 1 => {
                    self.selected_row += 1;
                    self.scroll();
                },
                KeyCode::Up if self.selected_row > 0 => {
                    self.selected_row -= 1;
                    self.scroll();
                }
                _ => {},
            }
        }
    }
}

impl Widget for SubgroupsWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut text = vec![];
        for (pos, subgroup) in self.subgroups[self.scroll_top..self.scroll_bottom].iter().enumerate() {
            let name = self.group.clone() + &subgroup[..];
            let span = Spans::from(vec![Span::styled({
                if name.len() > area.width as usize {
                    String::from("...") + &name[(name.len() - area.width as usize + 5)..]
                } else {
                    name
                }
            }, {
                if self.focused && pos + self.scroll_top == self.selected_row {
                    Style::default().bg(Color::Red)
                } else {
                    Style::default()
                }
            })]);
            text.push(span);
        }
        let paragraph = Paragraph::new(text)
            .block(Block::default()
            .title("Subgroups")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Green).fg(if self.focused { Color::Cyan } else { Color::White })))
            .wrap(Wrap { trim: true });
        paragraph.render(area, buf);
    }
}
