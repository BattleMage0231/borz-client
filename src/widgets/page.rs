use crossterm::event::{KeyCode, KeyEvent};
use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::symbols::Marker;
use tui::widgets::{Block, Borders, Widget};

use crate::widgets::account::AccountWidget;
use crate::widgets::bio::BioWidget;
use crate::widgets::location::LocationWidget;
use crate::widgets::messages::MessagesWidget;
use crate::widgets::subgroups::SubgroupsWidget;
use crate::widgets::threads::ThreadsWidget;
use crate::widgets::user::UserWidget;

#[derive(Debug, Clone)]
enum ActiveWidget {
    Location,
    Threads,
    Subgroups,
    Account,
    User,
    Bio,
    Messages,
}

#[derive(Debug, Clone)]
pub struct GroupPage {
    location_widget: LocationWidget,
    threads_widget: ThreadsWidget,
    subgroups_widget: SubgroupsWidget,
    account_widget: AccountWidget,
    active: ActiveWidget,
}

impl GroupPage {
    pub fn new() -> GroupPage {
        let mut gw = LocationWidget::new(String::from("/test1/test2"));
        gw.focus();
        GroupPage {
            location_widget: gw,
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
            active: ActiveWidget::Location,
        }
    }

    pub fn update(&mut self, key: KeyEvent) {
        if key.modifiers.is_empty() {
            if let KeyCode::Tab = key.code {
                match self.active {
                    ActiveWidget::Location => {
                        self.location_widget.unfocus();
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
                        self.location_widget.focus();
                        self.active = ActiveWidget::Location;
                    }
                    _ => panic!("Illegal active widget"),
                };
                return;
            }
        }
        match self.active {
            ActiveWidget::Location => self.location_widget.update(key),
            ActiveWidget::Threads => self.threads_widget.update(key),
            ActiveWidget::Subgroups => self.subgroups_widget.update(key),
            ActiveWidget::Account => self.account_widget.update(key),
            _ => panic!("Illegal active widget"),
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
        self.location_widget.render(chunks[0], buf);
        self.threads_widget.render(chunks[1], buf);
        self.subgroups_widget.render(chunks[2], buf);
        self.account_widget.render(chunks[3], buf);
    }
}

#[derive(Debug, Clone)]
pub struct UserPage {
    user_widget: UserWidget,
    bio_widget: BioWidget,
    active: ActiveWidget,
}

impl UserPage {
    pub fn new() -> UserPage {
        let mut uw = UserWidget::new(
            String::from("/test/test2"),
            String::from("BattleMage_"),
            123812739128937,
        );
        uw.focus();
        UserPage {
            user_widget: uw,
            bio_widget: BioWidget::new("This is a bio!".repeat(100)),
            active: ActiveWidget::User,
        }
    }

    pub fn update(&mut self, key: KeyEvent) {
        if key.modifiers.is_empty() {
            if let KeyCode::Tab = key.code {
                match self.active {
                    ActiveWidget::User => {
                        self.user_widget.unfocus();
                        self.bio_widget.focus();
                        self.active = ActiveWidget::Bio;
                    }
                    ActiveWidget::Bio => {
                        self.bio_widget.unfocus();
                        self.user_widget.focus();
                        self.active = ActiveWidget::User;
                    }
                    _ => panic!("Illegal active widget"),
                };
                return;
            }
        }
        match self.active {
            ActiveWidget::User => self.user_widget.update(key),
            ActiveWidget::Bio => self.bio_widget.update(key),
            _ => panic!("Illegal active widget"),
        }
    }

    pub fn resize(&mut self, area: &Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(area.clone());
    }
}

impl Widget for UserPage {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(area);
        self.user_widget.render(chunks[0], buf);
        self.bio_widget.render(chunks[1], buf);
    }
}

#[derive(Debug, Clone)]
pub struct ThreadPage {
    location_widget: LocationWidget,
    messages_widget: MessagesWidget,
    account_widget: AccountWidget,
    active: ActiveWidget,
}

impl ThreadPage {
    pub fn new() -> ThreadPage {
        let mut lw = LocationWidget::new(String::from("/path/test/Thread1"));
        lw.focus();
        ThreadPage {
            location_widget: lw,
            messages_widget: MessagesWidget::new(
                vec![
                    String::from("Leyang Zou"),
                    String::from("BattleMage_"),
                    String::from("Cheesecake"),
                ],
                vec![
                    vec![
                        String::from("This is a line of text"),
                        String::from("This is another line of text"),
                        String::from("This is a third line of text"),
                        String::from("This is a fourth line of text"),
                        String::from("This is a line of text"),
                        String::from("This is another line of text"),
                        String::from("This is a third line of text"),
                        String::from("This is a fourth line of text"),
                        String::from("This is a line of text"),
                        String::from("This is another line of text"),
                        String::from("This is a third line of text"),
                        String::from("This is a fourth line of text"),
                        String::from("This is a line of text"),
                        String::from("This is another line of text"),
                        String::from("This is a third line of text"),
                        String::from("This is a fourth line of text"),
                        String::from("This is a line of text"),
                        String::from("This is another line of text"),
                        String::from("This is a third line of text"),
                        String::from("This is a fourth line of text"),
                        String::from("This is a line of text"),
                        String::from("This is another line of text"),
                        String::from("This is a third line of text"),
                        String::from("This is a fourth line of text"),
                        String::from("This is a line of text"),
                        String::from("This is another line of text"),
                        String::from("This is a third line of text"),
                        String::from("This is a fourth line of text"),
                        String::from("This is a line of text"),
                        String::from("This is another line of text"),
                        String::from("This is a third line of text"),
                        String::from("This is a fourth line of text"),
                        String::from("This is a line of text"),
                        String::from("This is another line of text"),
                        String::from("This is a third line of text"),
                        String::from("This is a fourth line of text"),
                        String::from("This is a line of text"),
                        String::from("This is another line of text"),
                        String::from("This is a third line of text"),
                        String::from("This is a fourth line of text"),
                    ],
                    vec![String::new()],
                    vec![String::from("stO BruCENaN Orz")],
                ],
                String::from("BattleMage0231"),
            ),
            account_widget: AccountWidget::new(String::from("BattleMage_")),
            active: ActiveWidget::Location,
        }
    }

    pub fn update(&mut self, key: KeyEvent) {
        if key.modifiers.is_empty() {
            if let KeyCode::Tab = key.code {
                match self.active {
                    ActiveWidget::Location => {
                        self.location_widget.unfocus();
                        self.messages_widget.focus();
                        self.active = ActiveWidget::Messages;
                    }
                    ActiveWidget::Messages => {
                        self.messages_widget.unfocus();
                        self.account_widget.focus();
                        self.active = ActiveWidget::Account;
                    }
                    ActiveWidget::Account => {
                        self.account_widget.unfocus();
                        self.location_widget.focus();
                        self.active = ActiveWidget::Location;
                    }
                    _ => panic!("Illegal active widget"),
                };
                return;
            }
        }
        match self.active {
            ActiveWidget::Location => self.location_widget.update(key),
            ActiveWidget::Messages => self.messages_widget.update(key),
            ActiveWidget::Account => self.account_widget.update(key),
            _ => panic!("Illegal active widget"),
        }
    }

    pub fn resize(&mut self, area: &Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10),
                ]
                .as_ref(),
            )
            .split(area.clone());
        self.messages_widget.resize(&chunks[1]);
    }
}

impl Widget for ThreadPage {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints(
                [
                    Constraint::Percentage(10),
                    Constraint::Percentage(80),
                    Constraint::Percentage(10),
                ]
                .as_ref(),
            )
            .split(area);
        self.location_widget.render(chunks[0], buf);
        self.messages_widget.render(chunks[1], buf);
        self.account_widget.render(chunks[2], buf);
    }
}
