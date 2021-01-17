use crate::app::{App, AppPage};
use crate::widgets::page::GroupPage;
use crossterm::event::{KeyCode, KeyEvent};
use std::cmp::min;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Widget, Wrap};

#[derive(Debug, Clone)]
pub struct SubgroupsWidget {
    group: String,
    subgroups: Vec<String>,
    sbids: Vec<String>,
    selected_row: usize,
    focused: bool,
    scroll_top: usize,
    scroll_bottom: usize,
}

impl SubgroupsWidget {
    pub fn new(group: String, subgroups: Vec<String>, sbids: Vec<String>) -> SubgroupsWidget {
        SubgroupsWidget {
            group,
            subgroups,
            selected_row: 0,
            focused: false,
            scroll_top: 0,
            scroll_bottom: 0,
            sbids,
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

    pub fn update(&mut self, key: KeyEvent) -> Box<dyn for<'a> Fn(&'a mut App)> {
        if !self.focused {
            return Box::new(|_| {});
        }
        if key.modifiers.is_empty() && !self.subgroups.is_empty() {
            match key.code {
                KeyCode::Down if self.selected_row < self.subgroups.len() - 1 => {
                    self.selected_row += 1;
                    self.scroll();
                }
                KeyCode::Up if self.selected_row > 0 => {
                    self.selected_row -= 1;
                    self.scroll();
                }
                KeyCode::Enter => {
                    let id = self.sbids[self.selected_row].clone();
                    let sg = self.subgroups[self.selected_row].clone();
                    return Box::new(move |app| {
                        if let AppPage::Group(gp) = app.get_page().unwrap() {
                            let mut gp = gp.clone();
                            app.push_page(AppPage::Group(GroupPage::new(
                                gp.fetcher.child(id.clone()),
                                gp.group_path + "/" + &sg[..],
                                gp.username.clone(),
                            )));
                        } else {
                            panic!("Wrong page execution");
                        }
                    });
                }
                _ => {}
            };
            return Box::new(|_| {});
        }
        return Box::new(|_| {});
    }
}

impl Widget for SubgroupsWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut text = vec![];
        if self.subgroups.len() > 0 {
            for (pos, subgroup) in self.subgroups[self.scroll_top..self.scroll_bottom]
                .iter()
                .enumerate()
            {
                let name = self.group.clone() + &subgroup[..];
                let span = Spans::from(vec![Span::styled(
                    {
                        if name.len() > area.width as usize {
                            String::from("...") + &name[(name.len() - area.width as usize + 5)..]
                        } else {
                            name
                        }
                    },
                    {
                        if self.focused && pos + self.scroll_top == self.selected_row {
                            Style::default().bg(Color::Red)
                        } else {
                            Style::default()
                        }
                    },
                )]);
                text.push(span);
            }
        }
        let paragraph = Paragraph::new(text)
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
