use configparser::ini::Ini;
use std::fs;
use std::io::prelude::*;
use yansi::{Color, Paint};

const CONFIG_FILE: &str = "config.ini";

fn write_t(text: &str, color: yansi::Color) {
    match color {
        Color::Blue => print!("{}", Paint::blue(text)),
        Color::Green => print!("{}", Paint::green(text)),
        Color::Yellow => print!("{}", Paint::yellow(text)),
        Color::Magenta => print!("{}", Paint::magenta(text)),
        Color::Red => print!("{}", Paint::red(text)),
        _ => panic!("color not found"),
    }
}

fn match_term_color(color: &str) -> Color {
    match color {
        "BLUE" => Color::Blue,
        _ => panic!("color not found in config file"),
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
    config_file.load(CONFIG_FILE.to_string()).unwrap();

    let config_content = fs::read_to_string(CONFIG_FILE).unwrap();
    println!("{}", config_content);
    let color_desc = config_file.get("colors", "desc").unwrap();
    println!("{}", color_desc);
    write_t("test color\n", match_term_color(&color_desc));
}
