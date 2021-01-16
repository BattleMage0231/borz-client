use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use crate::widgets::page::GroupPage;

#[derive(Debug)]
pub enum AppPage {
    Home,
    Group(GroupPage),
    Thread,
}

#[derive(Debug)]
pub enum ActiveBlock {
    Group,
    Threads,
    Subgroups,
    Account,
}

#[derive(Debug)]
pub struct App {
    route: Vec<AppPage>,
    chars: Vec<char>,
}

impl App {
    pub fn new() -> App {
        App {
            route: Vec::new(),
            chars: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        self.route.push(AppPage::Group(GroupPage::new())); // debug
        if self.route.is_empty() {
            self.route.push(AppPage::Home);
        }
    }

    pub fn tick(&mut self) {
        match self.get_page().unwrap() {
            _ => {},
        }
    }

    pub fn update(&mut self, chr: KeyEvent) {
        match self.get_page().unwrap() {
            AppPage::Group(gp) => {
                gp.update(chr);
            }
            _ => {},
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
