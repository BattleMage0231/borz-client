#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use crossbeam_channel::{select, tick, unbounded, Receiver, RecvError};
use crossterm::cursor;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use crossterm::execute;
use crossterm::terminal;
use std::io;
use std::thread;
use std::time::Duration;
use std::sync::Mutex;
use tui::backend::CrosstermBackend;
use tui::Terminal;

mod api;
mod app;
mod ui;
mod widgets;

fn setup_terminal() {
    let mut stdout = io::stdout();
    execute!(stdout, terminal::EnterAlternateScreen).unwrap();
    execute!(stdout, cursor::Hide).unwrap();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    terminal::enable_raw_mode().unwrap();
}

fn cleanup_terminal() {
    let mut stdout = io::stdout();
    execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    execute!(stdout, terminal::LeaveAlternateScreen).unwrap();
    execute!(stdout, cursor::Show).unwrap();
    terminal::disable_raw_mode().unwrap();
}

fn setup_ui_events() -> Receiver<Event> {
    let (tx, rx) = unbounded();
    thread::spawn(move || loop {
        match tx.send(crossterm::event::read().unwrap()) {
            Ok(_) => {},
            Err(_) => {},  
        }
    });
    return rx;
}

fn main() {
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend).unwrap();
    setup_terminal();
    let ticker = tick(Duration::from_secs_f64(0.4));
    let ui_events_receiver = setup_ui_events();
    let mut app_instance = app::App::new();
    app_instance.start();
    ui::draw(&mut terminal, &mut app_instance);
    loop {
        select! {
            recv(ticker) -> _ => {
                app_instance.tick();
            }
            recv(ui_events_receiver) -> message => {
                match message {
                    Err(e) => {
                        println!("{}", e);
                    }
                    Ok(_) => (),
                }
                match message.unwrap() {
                    Event::Key(key_event) => {
                        if key_event.modifiers == KeyModifiers::CONTROL {
                            if let KeyCode::Char(c) = key_event.code {
                                if c == 'c' {
                                    break;
                                }
                            }
                        } else {
                            app_instance.update(key_event);
                        }
                    }
                    _ => (),
                }
            }
        };
        ui::draw(&mut terminal, &mut app_instance);
    }
    cleanup_terminal();
}
