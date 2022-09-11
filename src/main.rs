use configparser::ini::Ini;
use rusqlite::{params, Connection, Result};
use std::fs;
use std::io::prelude::*;
use yansi::{Color, Paint};

const CONFIG_FILE: &str = "config.ini";

#[derive(Debug)]
struct Scheduler {
    day: String,
    hour_start: String,
    hour_end: String,
    description: String,
}

fn write_t(text: &str, color: Color) {
    print!(
        "{}",
        match color {
            Color::Black => Paint::black(text).bold(),
            Color::Red => Paint::red(text).bold(),
            Color::Green => Paint::green(text).bold(),
            Color::Yellow => Paint::yellow(text).bold(),
            Color::Blue => Paint::blue(text).bold(),
            Color::Magenta => Paint::magenta(text).bold(),
            Color::Cyan => Paint::cyan(text).bold(),
            Color::White => Paint::white(text).bold(),
            Color::Default => Paint::default(text),
            _ => panic!("color not found"),
        }
    );
}

fn match_term_color(color: &str) -> Result<Color, &str> {
    match color {
        "BLACK" => Ok(Color::Black),
        "RED" => Ok(Color::Red),
        "GREEN" => Ok(Color::Green),
        "YELLOW" => Ok(Color::Yellow),
        "BLUE" => Ok(Color::Blue),
        "MAGENTA" => Ok(Color::Magenta),
        "CYAN" => Ok(Color::Cyan),
        "WHITE" => Ok(Color::White),
        _ => Err("color not found"),
    }
}

fn create_default_config() {
    let mut file = fs::File::create(CONFIG_FILE).expect("could not create file");
    file.write_all(b"[colors]\ndesc = BLUE\nhours = GREEN\n\n[database]\npath = ./data.db\n")
        .expect("could not write to file");
}

fn create_default_database(db_path: &str) -> Result<()> {
    let db = Connection::open(db_path).expect("unable to open database");
    db.execute("create table if not EXISTS scheduler (day TEXT, hour_start TEX, hour_end TEXT, description TEXT);", ()).unwrap();

    Ok(())
}

fn show_main_menu() {
    write_t("[1] ", Color::Green);
    write_t("Add new day\n", Color::Cyan);
    write_t("[2] ", Color::Green);
    write_t("Add new task made to a day\n", Color::Cyan);
    write_t("[3] ", Color::Green);
    write_t("Remove a task\n", Color::Cyan);
    print!("=> ");
    std::io::stdout().flush().unwrap();

    let mut response = String::new();
    std::io::stdin().read_line(&mut response).unwrap();
    let response: u8 = response.trim().parse().expect("unable to parse string to uint");
    println!("{}", response);
}

fn main() {
    if cfg!(windows) && !Paint::enable_windows_ascii() {
        Paint::disable();
    }

    let mut config_file = Ini::new();

    write_t("Daily ", Color::Green);
    write_t("Scheduler\n", Color::Blue);

    write_t("Write what you did ", Color::White);
    write_t("today", Color::Magenta);
    write_t(" to make your tomorrow ", Color::White);
    write_t("better!\n", Color::Yellow);

    // check config file and if not exists make a new one
    // change exists with try_exists when it exits from nightly build
    if !std::path::Path::new(CONFIG_FILE).exists() {
        write_t(
            "\n[error] config file not found, making a new one...\n",
            Color::Red,
        );
        create_default_config();
        write_t("[info] created config file!\n", Color::Yellow);
    }

    // load INI config file
    config_file.load(CONFIG_FILE).unwrap();

    // get and check db path
    let db_path = config_file.get("database", "path").unwrap();
    if !std::path::Path::new(&db_path).exists() {
        write_t(
            "[error] database not found, making a new one...\n",
            Color::Red,
        );
        match create_default_database(&db_path) {
            Ok(_) => write_t("[info] created database file\n", Color::Yellow),
            Err(_) => panic!("unable to create database file"),
        }
    }

    // get description and hours colors
    let color_desc = config_file
        .get("colors", "desc")
        .expect("could not find desc color in config file");
    let color_desc = match_term_color(&color_desc);
    let color_hours = config_file
        .get("colors", "hours")
        .expect("could not find hours color in config file");
    let color_hours = match_term_color(&color_hours);

    let color_desc = match color_desc {
        Ok(color) => color,
        Err(_) => panic!("desc color not found in config file"),
    };

    let color_hours = match color_hours {
        Ok(color) => color,
        Err(_) => panic!("hours color not found in config file"),
    };

    show_main_menu();
}
