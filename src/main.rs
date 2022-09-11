use configparser::ini::Ini;
use std::fs;
use std::io::prelude::*;
use yansi::{Color, Paint};

const CONFIG_FILE: &str = "config.ini";

fn write_t(text: &str, color: Color) {
    print!(
        "{}",
        match color {
            Color::Black => Paint::black(text),
            Color::Red => Paint::red(text),
            Color::Green => Paint::green(text),
            Color::Yellow => Paint::yellow(text),
            Color::Blue => Paint::blue(text),
            Color::Magenta => Paint::magenta(text),
            Color::Cyan => Paint::cyan(text),
            Color::White => Paint::white(text),
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
    file.write_all(b"[colors]\ndesc = BLUE\nhours = GREEN\n")
        .expect("could not write to file");

    /* config.ini:
    [colors]
    desc = BLUE
    hours = GREEN
    */
}

fn main() {
    if cfg!(windows) && !Paint::enable_windows_ascii() {
        Paint::disable();
    }

    let mut config_file = Ini::new();

    write_t("Daily ", Color::Green);
    write_t("Scheduler\n", Color::Blue);

    print!("Write what you did ");
    write_t("today", Color::Magenta);
    print!(" to make your tomorrow ");
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

    write_t("desc color ", color_desc);
    write_t("hours color\n", color_hours);
}
