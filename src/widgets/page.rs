use crossterm::event::{KeyCode, KeyEvent};
use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::symbols::Marker;
use tui::widgets::{Block, Borders, Widget};

use crate::widgets::account::AccountWidget;
use crate::widgets::group::GroupWidget;
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
                vec![
                    String::from("Hello, World!") + &("12345678901234567890".repeat(100))[..],
                    String::from("Goodbye, World!"),
                    String::from("Hello, World!") + &("12345678901234567890".repeat(100))[..],
                    String::from("Goodbye, World!"),
                    String::from("Hello, World!") + &("12345678901234567890".repeat(100))[..],
                    String::from("Goodbye, World!"),
                    String::from("Hello, World!") + &("12345678901234567890".repeat(100))[..],
                    String::from("Goodbye, World!"),
                    String::from("Hello, World!") + &("12345678901234567890".repeat(100))[..],
                    String::from("Goodbye, World!"),
                    String::from("Hello, World!") + &("12345678901234567890".repeat(100))[..],
                    String::from("Goodbye, World!"),
                    String::from("Hello, World!") + &("12345678901234567890".repeat(100))[..],
                    String::from("Goodbye, World!"),
                    String::from("Hello, World!") + &("12345678901234567890".repeat(100))[..],
                    String::from("Goodbye, World!"),
                    String::from("Hello, World!") + &("12345678901234567890".repeat(100))[..],
                    String::from("Goodbye, World!"),
                    String::from("Hello, World!") + &("12345678901234567890".repeat(100))[..],
                    String::from("Goodbye, World!"),
                    String::from("Hello, World!") + &("12345678901234567890".repeat(100))[..],
                    String::from("Goodbye, World!"),
                    String::from("Hello, World!") + &("12345678901234567890".repeat(100))[..],
                    String::from("Goodbye, World!"),
                    String::from("Hello, World!") + &("12345678901234567890".repeat(100))[..],
                    String::from("Goodbye, World!"),
                ],
                vec![
                    String::from("BattleMage_"),
                    String::from("foobar"),
                    String::from("BattleMage_"),
                    String::from("foobar"),
                    String::from("BattleMage_"),
                    String::from("foobar"),
                    String::from("BattleMage_"),
                    String::from("foobar"),
                    String::from("BattleMage_"),
                    String::from("foobar"),
                    String::from("BattleMage_"),
                    String::from("foobar"),
                    String::from("BattleMage_"),
                    String::from("foobar"),
                    String::from("BattleMage_"),
                    String::from("foobar"),
                    String::from("BattleMage_"),
                    String::from("foobar"),
                    String::from("BattleMage_"),
                    String::from("foobar"),
                    String::from("BattleMagxe_"),
                    String::from("foobar"),
                    String::from("BattleMage_"),
                    String::from("foobar"),
                    String::from("BattleMage_"),
                    String::from("foobar"),
                ],
            ),
            subgroups_widget: SubgroupsWidget::new(
                String::from("/test/hi"),
                vec![
                    String::from("hi"),
                    String::from("bye"),
                    String::from("hello") + &("12345678901234567890".repeat(100))[..],
                    String::from("hello"),
                    String::from("hello1"),
                    String::from("hello2"),
                    String::from("hello3"),
                    String::from("hello4"),
                    String::from("hello5"),
                    String::from("hello6"),
                    String::from("hello7"),
                    String::from("hello8"),
                ],
            ),
            account_widget: AccountWidget::new(String::from("BattleMage_")),
            active: ActiveWidget::Group,
        }
    }

    pub fn update(&mut self, key: KeyEvent) {
        if key.modifiers.is_empty() {
            if let KeyCode::Char(c) = key.code {
                if c == 'x' {
                    match self.active {
                        ActiveWidget::Group => {
                            self.group_widget.unfocus();
                            self.threads_widget.focus();
                            self.active = ActiveWidget::Threads;
                        }
                        ActiveWidget::Threads => {
                            self.threads_widget.unfocus();
                            self.subgroups_widget.focus();
                            self.active = ActiveWidget::Subgroups;
                        }
                        ActiveWidget::Subgroups => {
                            self.subgroups_widget.unfocus();
                            self.account_widget.focus();
                            self.active = ActiveWidget::Account;
                        }
                        ActiveWidget::Account => {
                            self.account_widget.unfocus();
                            self.group_widget.focus();
                            self.active = ActiveWidget::Group;
                        }
                    };
                    return;
                }
            }
        }
        match self.active {
            ActiveWidget::Group => self.group_widget.update(key),
            ActiveWidget::Threads => self.threads_widget.update(key),
            ActiveWidget::Subgroups => self.subgroups_widget.update(key),
            ActiveWidget::Account => self.account_widget.update(key),
        }
    }

    pub fn resize(&mut self, area: &Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(60),
                    Constraint::Percentage(20),
                    Constraint::Percentage(10),
                ]
                .as_ref(),
            )
            .split(area.clone());
        self.threads_widget.resize(&chunks[1]);
        self.subgroups_widget.resize(&chunks[2]);
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
                ]
                .as_ref(),
            )
            .split(area);
        self.group_widget.render(chunks[0], buf);
        self.threads_widget.render(chunks[1], buf);
        self.subgroups_widget.render(chunks[2], buf);
        self.account_widget.render(chunks[3], buf);
    }
}
