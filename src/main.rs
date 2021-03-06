use clap::ArgMatches;
use crossbeam_channel::{select, tick, unbounded, Receiver};
use crossterm::cursor;
use crossterm::event::{Event, KeyCode, KeyModifiers};
use crossterm::execute;
use crossterm::terminal;
use dirs::home_dir;
use json::JsonValue;
use lazy_static::lazy_static;
use rpassword::read_password;
use std::fs;
use std::io;
use std::path::Path;
use std::thread;
use std::time::Duration;
use tui::backend::CrosstermBackend;
use tui::Terminal;
use url::Url;

mod api;
mod app;
mod args;
mod ui;
mod widgets;

lazy_static! {
    pub static ref HOME_DIR: String = home_dir().unwrap().into_os_string().into_string().unwrap();
    pub static ref CONFIG_FOLDER_PATH: String = format!("{}/.config/Borz", *HOME_DIR);
    pub static ref CONFIG_FILE_PATH: String = format!("{}/.config/Borz/config.json", *HOME_DIR);
    pub static ref TOP_LEVEL_ID: String = String::from("U3ViZ3JvdXBOb2RlOjE=");
}

fn load_config() -> JsonValue {
    fs::create_dir_all(CONFIG_FOLDER_PATH.clone()).unwrap();
    if !Path::new(&CONFIG_FILE_PATH.clone()).is_file() {
        fs::write(CONFIG_FILE_PATH.clone(), "{}").unwrap();
    }
    let content = fs::read_to_string(CONFIG_FILE_PATH.clone()).unwrap();
    match json::parse(&content[..]) {
        Err(_) => {
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

fn launch_app(args: ArgMatches, config: JsonValue) {
    if !config.has_key("token")
        || !config.has_key("refresh_token")
        || !config.has_key("username")
        || !config.has_key("server")
    {
        println!("Please run Borz login or Borz signup to log in first");
        return;
    }
    let backend = CrosstermBackend::new(io::stdout());
    let mut terminal = Terminal::new(backend).unwrap();
    setup_terminal();
    let ticker = tick(Duration::from_secs_f64(0.4));
    let ui_events_receiver = setup_ui_events();
    let mut app_instance = app::App::new(args, config);
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
                            let cont = app_instance.update(key_event);
                            if !cont {
                                break;
                            }
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

fn read_line() -> String {
    let mut str: String = String::new();
    io::stdin().read_line(&mut str).unwrap();
    return String::from(str.trim());
}

fn main() {
    let args = args::parse_args();
    let config = load_config();
    match args.subcommand_name() {
        None => launch_app(args, config),
        Some(name) => match name {
            "clean" => {
                fs::remove_file(CONFIG_FILE_PATH.clone()).unwrap();
                fs::remove_dir(CONFIG_FOLDER_PATH.clone()).unwrap();
                println!("Successfully removed all cached and config data.");
            }
            "login" => {
                println!("Enter the address of the server:");
                let server = Url::parse(&read_line()[..]).unwrap();
                println!("Enter your username:");
                let username = read_line();
                println!("Enter your password:");
                let password: String = read_password().unwrap();
                if username.is_empty() || password.is_empty() {
                    println!("Username and password cannot be empty!");
                    return;
                }
                let mut fetcher = api::fetch::APIFetcher::new(server.clone(), TOP_LEVEL_ID.clone());
                let res = fetcher.mutate_auth(username, password);
                let data = res.data.unwrap();
                let token_auth = data.token_auth.unwrap();
                if !token_auth.success.unwrap() {
                    println!("Unsuccessful login");
                    return;
                }
                let token = token_auth.token.unwrap();
                let refresh_token = token_auth.refresh_token.unwrap();
                fs::write(
                        CONFIG_FILE_PATH.clone(),
                        format!(
                            "{{\"token\": \"{}\", \"refresh_token\": \"{}\", \"username\": \"{}\", \"server\": \"{}\"}}",
                            token, refresh_token, token_auth.user.unwrap().username, server.into_string(),
                        )
                    ).unwrap();
                println!("You have successfully logged in!");
            }
            "logout" => {
                if Path::new(&CONFIG_FILE_PATH.clone()).is_file() {
                    fs::write(CONFIG_FILE_PATH.clone(), "{}").unwrap();
                }
                println!("Successfully logged out.");
            }
            "signup" => {
                println!("Enter the address of the server:");
                let server = Url::parse(&read_line()[..]).unwrap();
                println!("Enter your email:");
                let email = read_line();
                println!("Enter a username:");
                let username = read_line();
                println!("Enter a password:");
                let password: String = read_password().unwrap();
                println!("Re-enter your password:");
                let check: String = read_password().unwrap();
                if password != check {
                    println!("Entered passwords must match!");
                    return;
                }
                let fetcher = api::fetch::APIFetcher::new(server, TOP_LEVEL_ID.clone());
                let res = fetcher.mutate_register(email, username, password);
                if !res.data.unwrap().register.unwrap().success.unwrap() {
                    println!("Your request was rejected by the server. Please make sure you have a valid username, email, and a strong password.");
                    return;
                }
                println!("You have successfully signed up for Borz. Please check your email for more instructions");
            }
            "verify" => {
                println!("Enter the address of the server:");
                let server = Url::parse(&read_line()[..]).unwrap();
                println!("Enter the key from your email:");
                let key = read_line();
                let fetcher = api::fetch::APIFetcher::new(server, TOP_LEVEL_ID.clone());
                let res = fetcher.mutate_verify(key);
                if !res.data.unwrap().verify_account.unwrap().success.unwrap() {
                    println!("Your token was incorrect");
                    return;
                }
                println!("You have successfully created an account. You may now log in by running borz login");
            }
            _ => panic!("Unknown argument"),
        },
    }
}
