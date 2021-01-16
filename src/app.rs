use crate::widgets::page::{GroupPage, ThreadPage, UserPage};
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent};

#[derive(Debug)]
pub enum AppPage {
    User(UserPage),
    Group(GroupPage),
    Thread(ThreadPage),
}

#[derive(Debug)]
pub struct App {
    route: Vec<AppPage>,
}

impl App {
    pub fn new() -> App {
        App { route: Vec::new() }
    }

    pub fn start(&mut self) {
        self.route.push(AppPage::Thread(ThreadPage::new())); // debug
    }

    pub fn tick(&mut self) {
        match self.get_page().unwrap() {
            _ => {}
        }
    }

    pub fn update(&mut self, chr: KeyEvent) {
        match self.get_page().unwrap() {
            AppPage::Group(gp) => {
                gp.update(chr);
            }
            AppPage::User(up) => {
                up.update(chr);
            }
            AppPage::Thread(tp) => {
                tp.update(chr);
            }
        }
    }

    pub fn push_page(&mut self, page: AppPage) {
        self.route.push(page);
    }

    pub fn pop_page(&mut self) -> Option<AppPage> {
        if self.route.is_empty() {
            None
        } else {
            Some(self.route.pop().unwrap())
        }
    }

    pub fn get_page(&mut self) -> Option<&mut AppPage> {
        if self.route.is_empty() {
            None
        } else {
            Some(self.route.last_mut().unwrap())
        }
    }
}
