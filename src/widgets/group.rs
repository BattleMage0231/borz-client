use tui::buffer::Buffer;
use tui::symbols::Marker;
use tui::widgets::{Widget, Block, Borders, Paragraph, Wrap};
use tui::layout::{Layout, Constraint, Direction, Rect};
use crossterm::event::KeyEvent;

#[derive(Debug, Clone)]
pub struct GroupWidget {
    group: String,
    focused: bool,
}

impl GroupWidget {
    pub fn new(group: String) -> GroupWidget {
        GroupWidget {
            group,
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

impl Widget for GroupWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let paragraph = Paragraph::new(self.group)
            .block(Block::default().title("Group").borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        paragraph.render(area, buf);
    }
}
