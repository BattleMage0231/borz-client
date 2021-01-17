#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_variables)]

use crossbeam_channel::{select, tick, unbounded, Receiver, RecvError};
use crossterm::cursor;
use crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers, MouseEvent};
use crossterm::execute;
use crossterm::terminal;
use std::io;
use std::io::{Read, Write, BufRead};
use std::sync::Mutex;
use std::thread;
use std::time::Duration;
use std::fs;
use std::path;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use clap::{Arg, App, SubCommand, ArgMatches};
use json::{JsonValue, parse};
use rpassword::read_password;

mod api;
mod app;
mod ui;
mod widgets;
mod args;

fn load_config() -> JsonValue {
    fs::create_dir_all("~/.config/Borz/").unwrap();
    if !path::Path::new("~/.config/Borz/config.json").is_file() {
        fs::write("~/.config/Borz/config.json", "{}").unwrap();
    }
    let content = fs::read_to_string("~/.config/Borz/config.json").unwrap();
    match json::parse(&content[..]) {
        Err(err) => {
            panic!("Bad JSON")
        }
        Ok(obj) => {
            return obj;
        }
    }
}

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
            Ok(_) => {}
            Err(_) => {}
        }
    });
    return rx;
}

fn launch_app(args: ArgMatches) {
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

fn main() {
    let args = args::parse_args();
    let config = load_config();
    match args.subcommand_name() {
        None => launch_app(args),
        Some(name) => match name {
            "clean" => {
                fs::remove_file("~/.config/Borz/config.json").unwrap();
                fs::remove_dir("~/.config/Borz").unwrap();
                println!("Successfully removed all cached and config data.");
            },
            "login" => {
                println!("Enter your username:");
                let mut username: String = String::new();
                io::stdin().read_line(&mut username).unwrap();
                username = String::from(username.trim());
                println!("Enter your password:");
                let password: String = read_password().unwrap();
                if username.is_empty() || password.is_empty() {
                    println!("Username and password cannot be empty!");
                    return;
                }
                println!("LOGGED IN WITH \"{}\" and \"{}\"", username, password);
            },
            "logout" => {
                println!("LOGOUT");
            },
            "signup" => {
                println!("Enter a username:");
                let mut username: String = String::new();
                io::stdin().read_line(&mut username).unwrap();
                username = String::from(username.trim());
                println!("Enter a password:");
                let password: String = read_password().unwrap();
                println!("Re-enter your password:");
                let check: String = read_password().unwrap();
                if password != check {
                    println!("Entered passwords must match!");
                    return;
                }
                println!("SIGNED UP WITH \"{}\" and \"{}\"", username, password);
            },
            _ => panic!("Unknown argument"),
        }
    }
}
