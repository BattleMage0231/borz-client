use crate::app::{App, AppPage};
use tui::backend::Backend;
use tui::layout::{Constraint, Layout};
use tui::Terminal;

pub fn draw<B: Backend>(terminal: &mut Terminal<B>, app: &mut App) {
    match app.get_page().as_mut().unwrap() {
        AppPage::Group(gp) => {
            gp.resize(&terminal.get_frame().size());
            terminal
                .draw(|f| {
                    let chunk = Layout::default()
                        .constraints(vec![Constraint::Percentage(100)])
                        .split(f.size())[0];
                    f.render_widget(gp.clone(), chunk);
                })
                .unwrap();
        }
        AppPage::User(up) => {
            up.resize(&terminal.get_frame().size());
            terminal
                .draw(|f| {
                    let chunk = Layout::default()
                        .constraints(vec![Constraint::Percentage(100)])
                        .split(f.size())[0];
                    f.render_widget(up.clone(), chunk);
                })
                .unwrap();
        }
        AppPage::Thread(tp) => {
            tp.resize(&terminal.get_frame().size());
            terminal
                .draw(|f| {
                    let chunk = Layout::default()
                        .constraints(vec![Constraint::Percentage(100)])
                        .split(f.size())[0];
                    f.render_widget(tp.clone(), chunk);
                })
                .unwrap();
        }
    }
}
