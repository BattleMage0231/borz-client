use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout, Rect};
use tui::{Frame, Terminal};
use crate::app::{App, AppPage};
use crate::widgets::page::GroupPage;

pub fn draw<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) {
    match app.get_page().as_mut().unwrap() {
        AppPage::Group(gp) => {
            terminal.draw(|f| {
                let chunk = Layout::default()
				    .constraints(vec![Constraint::Percentage(100)])
				    .split(f.size())[0];
                f.render_widget(gp.clone(), chunk);
            }).unwrap();
        },
        _ => (),
    }
}
