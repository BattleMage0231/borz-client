use tui::buffer::Buffer;
use tui::symbols::Marker;
use tui::widgets::{Widget, Block, Borders};
use tui::layout::{Layout, Constraint, Direction, Rect};
use crossterm::event::KeyEvent;

use crate::widgets::group::GroupWidget;
use crate::widgets::account::AccountWidget;
use crate::widgets::subgroups::SubgroupsWidget;
use crate::widgets::threads::ThreadsWidget;

#[derive(Debug, Clone)]
enum ActiveWidget {
    Group,
    Threads,
    Subgroups,
    Account,
}

#[derive(Debug, Clone)]
pub struct GroupPage {
    group_widget: GroupWidget,
    threads_widget: ThreadsWidget,
    subgroups_widget: SubgroupsWidget,
    account_widget: AccountWidget,
    active: ActiveWidget,
}

impl GroupPage {
    pub fn new() -> GroupPage {
        let mut gw = GroupWidget::new(String::from("/test1/test2"));
        gw.focus();
        GroupPage {
            group_widget: gw,
            threads_widget: ThreadsWidget::new(
                vec![String::from("Hello, World!"), String::from("Goodbye, World!")],
                vec![String::from("BattleMage_"), String::from("foobar")],
            ),
            subgroups_widget: SubgroupsWidget::new(vec![String::from("hi"), String::from("bye")]),
            account_widget: AccountWidget::new(String::from("BattleMage_")),
            active: ActiveWidget::Group,
        }
    }

    pub fn update(&mut self, key: KeyEvent) {
        match self.active {
            ActiveWidget::Group => self.group_widget.update(key),
            ActiveWidget::Threads => self.threads_widget.update(key),
            ActiveWidget::Subgroups => self.subgroups_widget.update(key),
            ActiveWidget::Account => self.account_widget.update(key),
        }
    }
}

impl Widget for GroupPage {
    fn render(self, area: Rect, buf: &mut Buffer) { 
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(60),
                    Constraint::Percentage(20),
                    Constraint::Percentage(10),
                ].as_ref()
            )
            .split(area);
        self.group_widget.render(chunks[0], buf);
        self.threads_widget.render(chunks[1], buf);
        self.subgroups_widget.render(chunks[2], buf);
        self.account_widget.render(chunks[3], buf);
    }
}
