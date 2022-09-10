extern crate term;

use std::io::prelude::*;
use std::fs;
use configparser::ini::Ini;

const CONFIG_FILE: &str = "config.ini";

fn write_t(term: &mut Box<term::StdoutTerminal>, text: &str, color: term::color::Color) {
    term.fg(color).expect("could not set the color");
    write!(term, "{}", text).expect("could not write to stdout");
    term.reset().unwrap();
}

fn create_default_config() {
    let mut file = fs::File::create(CONFIG_FILE).expect("could not create file");
    file.write_all(b"[colors]\ndesc = BLUE\nhours = GREEN").expect("could not write to file");
    
    /* config.ini:
    [colors]
    desc = BLUE
    hours = GREEN
    */
}

fn match_term_color(color: &str) -> term::color::Color {
    match color {
        "BLUE" => { term::color::BLUE },
        "GREEN" => { term::color::GREEN },
        _ => panic!("color in config file not found!"),
    }
}

fn main() {
    let mut t = term::stdout().expect("could not setup term::stdout");
    let mut config_file = Ini::new();
    let mut config_content = String::new();
    
    write_t(&mut t, "Daily ", term::color::GREEN);
    write_t(&mut t, "Scheduler\n", term::color::BLUE);
    
    print!("Write what you did ");
    write_t(&mut t, "today", term::color::MAGENTA);
    print!(" to make your tomorrow ");
    write_t(&mut t, "better!\n", term::color::YELLOW);
    
    // check config file and if not exists make a new one
    // change exists with try_exists when it exits from nightly build
    if !std::path::Path::new(CONFIG_FILE).exists() {
        write_t(&mut t, "\n[error] config file not found, making a new one...\n", term::color::RED);
        create_default_config();
        write_t(&mut t, "[info] created config file!\n", term::color::YELLOW);
    }
    
    // load INI config file
    config_file.load(CONFIG_FILE.to_string()).unwrap();
    
    config_content = fs::read_to_string(CONFIG_FILE).unwrap();
    println!("{}", config_content);
    let color_desc = config_file.get("colors", "desc").unwrap();
    println!("{}", color_desc);
    write_t(&mut t, "test color\n", match_term_color(&color_desc));
}
