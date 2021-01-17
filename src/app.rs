use crate::api::fetch::APIFetcher;
use crate::widgets::page::{GroupPage, ThreadPage, UserPage};
use crate::{API_ENDPOINT, TOP_LEVEL_ID};
use clap::ArgMatches;
use crossterm::event::KeyEvent;
use json::JsonValue;

#[derive(Debug)]
pub enum AppPage {
    User(UserPage),
    Group(GroupPage),
    Thread(ThreadPage),
}

#[derive(Debug)]
pub struct App<'a> {
    route: Vec<AppPage>,
    args: ArgMatches<'a>,
    config: JsonValue,
}

impl<'a> App<'a> {
    pub fn new(args: ArgMatches<'a>, config: JsonValue) -> App {
        App {
            route: Vec::new(),
            args,
            config,
        }
    }

    pub fn start(&mut self) {
        self.route.push(AppPage::Group(GroupPage::new(
            APIFetcher::new(API_ENDPOINT.clone(), TOP_LEVEL_ID.clone()),
            String::from("/Universe"),
            self.config["username"].to_string(),
        )));
    }

    pub fn tick(&mut self) {
        match self.get_page().unwrap() {
            _ => {}
        }
    }

    pub fn update(&mut self, chr: KeyEvent) -> bool {
        if self.route.is_empty() {
            return false;
        }
        let closure = match self.get_page().unwrap() {
            AppPage::Group(gp) => gp.update(chr),
            AppPage::User(up) => up.update(chr),
            AppPage::Thread(tp) => tp.update(chr),
        };
        closure(self);
        return !self.route.is_empty();
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
