use crate::app::App;
use crossterm::event::KeyEvent;
use tui::buffer::Buffer;
use tui::layout::Rect;
use tui::style::{Color, Style};
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph, Widget, Wrap};

#[derive(Debug, Clone)]
pub struct UserWidget {
    username: String,
    join_time: u64,
    focused: bool,
}

impl UserWidget {
    pub fn new(username: String, join_time: u64) -> UserWidget {
        UserWidget {
            username,
            join_time,
            focused: false,
        }
    }

    pub fn focus(&mut self) {
        self.focused = true;
    }

    pub fn unfocus(&mut self) {
        self.focused = false;
    }

    pub fn update(&mut self, key: KeyEvent) -> Box<dyn for<'a> Fn(&'a mut App)> {
        return Box::new(|_| {});
    }
}

impl Widget for UserWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut content = vec![];
        content.push(Spans::from(vec![
            Span::raw("User "),
            Span::styled(self.username, Style::default().fg(Color::Blue)),
        ]));
        content.push(Spans::from(vec![
            Span::raw("Joined "),
            Span::styled(self.join_time.to_string(), Style::default().fg(Color::Blue)),
        ]));
        let paragraph = Paragraph::new(content)
            .block(Block::default().title("User").borders(Borders::ALL).style(
                Style::default().bg(Color::Green).fg(if self.focused {
                    Color::Cyan
                } else {
                    Color::White
                }),
            ))
            .wrap(Wrap { trim: true });
        paragraph.render(area, buf);
    }
}
