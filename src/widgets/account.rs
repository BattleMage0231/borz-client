use tui::buffer::Buffer;
use tui::symbols::Marker;
use tui::widgets::{Widget, Block, Borders, Paragraph, Wrap};
use tui::layout::{Layout, Constraint, Direction, Rect};
use crossterm::event::KeyEvent;

#[derive(Debug, Clone)]
pub struct AccountWidget {
    account: String,
    focused: bool,
}

impl AccountWidget {
    pub fn new(account: String) -> AccountWidget {
        AccountWidget {
            account,
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

impl Widget for AccountWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let paragraph = Paragraph::new(format!("You are currently known as {}", self.account))
            .block(Block::default().title("Account").borders(Borders::ALL))
            .wrap(Wrap { trim: true });
        paragraph.render(area, buf);
    }
}
