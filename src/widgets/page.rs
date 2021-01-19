use crossterm::event::{KeyCode, KeyEvent};
use tui::buffer::Buffer;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::widgets::Widget;

use crate::api::fetch::APIFetcher;
use crate::app::{App, AppPage};
use crate::widgets::account::AccountWidget;
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
    //Bio,
    Messages,
}

#[derive(Debug, Clone)]
pub struct ThreadPage {
    location_widget: LocationWidget,
    messages_widget: MessagesWidget,
    account_widget: AccountWidget,
    active: ActiveWidget,
    new: bool,
    pub fetcher: APIFetcher,
    pub group_path: String,
    pub thread_id: String,
    pub username: String,
}

fn split_line_vec(s: String) -> Vec<String> {
    let mut v = vec![String::new()];
    for chr in s.chars() {
        if chr == '\n' {
            v.push(String::new());
        } else {
            v.last_mut().unwrap().push(chr);
        }
    }
    return v;
}

impl ThreadPage {
    pub fn new(
        mut fetcher: APIFetcher,
        group_path: String,
        thread_id: String,
        username: String,
        new: bool,
    ) -> ThreadPage {
        let mut lw = LocationWidget::new(group_path.clone());
        lw.focus();
        let res = fetcher
            .query_thread_content(thread_id.clone())
            .data
            .unwrap()
            .thread
            .unwrap();
        let mut authors = vec![];
        let mut content: Vec<Vec<String>> = vec![];
        authors.push(res.author.username);
        content.push(split_line_vec(res.content));
        for n in res.replies.edges {
            let node = n.unwrap().node.unwrap();
            authors.push(node.author.username);
            content.push(split_line_vec(node.content));
        }
        ThreadPage {
            location_widget: lw,
            messages_widget: MessagesWidget::new(authors, content, username.clone(), new),
            account_widget: AccountWidget::new(username.clone()),
            active: ActiveWidget::Location,
            new,
            fetcher,
            group_path,
            username,
            thread_id,
        }
    }

    pub fn update(&mut self, key: KeyEvent) -> Box<dyn for<'a> Fn(&'a mut App)> {
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
                return Box::new(|_| {});
            } else if let KeyCode::Esc = key.code {
                return Box::new(|app| {
                    app.pop_page().unwrap();
                });
            }
        }
        return match self.active {
            ActiveWidget::Location => self.location_widget.update(key),
            ActiveWidget::Messages => self.messages_widget.update(key),
            ActiveWidget::Account => self.account_widget.update(key),
            _ => panic!("Illegal active widget"),
        };
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

#[derive(Debug, Clone)]
pub struct GroupPage {
    location_widget: LocationWidget,
    threads_widget: ThreadsWidget,
    subgroups_widget: SubgroupsWidget,
    account_widget: AccountWidget,
    active: ActiveWidget,
    pub fetcher: APIFetcher,
    pub group_path: String,
    pub username: String,
}

impl GroupPage {
    pub fn new(mut fetcher: APIFetcher, group_path: String, username: String) -> GroupPage {
        let mut gw = LocationWidget::new(group_path.clone());
        gw.focus();
        let subgroups = fetcher.query_subgroups();
        let mut sb_list = vec![];
        let mut sbid_list = vec![];
        for subgroup in subgroups.data.unwrap().subgroup.unwrap().child_group.edges {
            let node = subgroup.unwrap().node.unwrap();
            sb_list.push(node.name);
            sbid_list.push(node.id);
        }
        let threads = fetcher.query_threads();
        let mut title_list = vec![];
        let mut tid_list = vec![];
        let mut author_list = vec![];
        let mut aid_list = vec![];
        for thread in threads.data.unwrap().subgroup.unwrap().threads.edges {
            let node = thread.unwrap().node.unwrap();
            title_list.push(node.title);
            tid_list.push(node.id);
            let aut = node.author;
            author_list.push(aut.username);
            aid_list.push(aut.id);
        }
        GroupPage {
            location_widget: gw,
            threads_widget: ThreadsWidget::new(title_list, tid_list, author_list, aid_list),
            subgroups_widget: SubgroupsWidget::new(group_path.clone(), sb_list, sbid_list),
            account_widget: AccountWidget::new(username.clone()),
            active: ActiveWidget::Location,
            fetcher,
            username,
            group_path,
        }
    }

    pub fn update(&mut self, key: KeyEvent) -> Box<dyn for<'a> Fn(&'a mut App)> {
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
                return Box::new(|_| {});
            } else if let KeyCode::Esc = key.code {
                return Box::new(|app| {
                    app.pop_page().unwrap();
                });
            } else if let KeyCode::Home = key.code {
                return Box::new(|app| {
                    if let AppPage::Group(_gp) = app.get_page().unwrap() {
                        /* Creating a new thread is not yet implemented
                        app.push_page(AppPage::Thread(ThreadPage::new(
                            gp.fetcher.clone(),
                            gp.group_path + "/Untitled",

                        )));
                        */
                    } else {
                        panic!("Wrong page execution");
                    }
                });
            }
        }
        return match self.active {
            ActiveWidget::Location => self.location_widget.update(key),
            ActiveWidget::Threads => self.threads_widget.update(key),
            ActiveWidget::Subgroups => self.subgroups_widget.update(key),
            ActiveWidget::Account => self.account_widget.update(key),
            _ => panic!("Illegal active widget"),
        };
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
    //bio_widget: BioWidget,
    active: ActiveWidget,
    pub fetcher: APIFetcher,
}

impl UserPage {
    pub fn new(mut fetcher: APIFetcher, user_id: String) -> UserPage {
        let res = fetcher.query_user(user_id).data.unwrap().user.unwrap();
        let username = res.username;
        let joined = res.date_joined.unwrap().parse::<u64>().unwrap();
        let mut uw = UserWidget::new(username, joined);
        uw.focus();
        UserPage {
            user_widget: uw,
            //bio_widget: BioWidget::new("This is a bio!".repeat(100)),
            active: ActiveWidget::User,
            fetcher,
        }
    }

    pub fn update(&mut self, key: KeyEvent) -> Box<dyn for<'a> Fn(&'a mut App)> {
        if key.modifiers.is_empty() {
            if let KeyCode::Esc = key.code {
                return Box::new(|app| {
                    app.pop_page().unwrap();
                });
            }
            if let KeyCode::Tab = key.code {
                /*
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
                */
                return Box::new(|_| {});
            }
        }
        return match self.active {
            ActiveWidget::User => self.user_widget.update(key),
            //ActiveWidget::Bio => self.bio_widget.update(key),
            _ => panic!("Illegal active widget"),
        };
    }

    pub fn resize(&mut self, _area: &Rect) {}
}

impl Widget for UserPage {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Percentage(30), Constraint::Percentage(70)].as_ref())
            .split(area);
        self.user_widget.render(chunks[0], buf);
        //self.bio_widget.render(chunks[1], buf);
    }
}
