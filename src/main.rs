mod config;
mod database;
mod terminal;

use yansi::{Paint, Color};

const CONFIG_FILE: &str = "config.ini";

fn show_main_menu() {
    terminal::write("[1] ", Color::Green);
    terminal::write("Add new day\n", Color::Cyan);
    terminal::write("[2] ", Color::Green);
    terminal::write("Add new task made to a day\n", Color::Cyan);
    terminal::write("[3] ", Color::Green);
    terminal::write("Remove a task\n", Color::Cyan);
    print!("=> ");

    let response: u8 = terminal::ask_input()
        .trim()
        .parse()
        .expect("unable to parse string to uint");

    match response {
        1 => terminal::write("selected 1\n", Color::Blue),
        _ => terminal::write("option not found\n", Color::Red),
    }
}

fn main() {
    if cfg!(windows) && !Paint::enable_windows_ascii() {
        Paint::disable();
    }

    let mut config_file = config::Config::new(CONFIG_FILE);
    if !config_file.check_path() {
        terminal::write(
            "\n[error] config file not found, making a new one...\n",
            Color::Red,
        );
        config_file.create_default_config();
        terminal::write("[info] created config file!\n", Color::Yellow);
    }

    if let Err(e) = config_file.load_config() {
        terminal::write(e, Color::Red);
    }

    let db_path = config_file.get("database", "path").unwrap();
    let db = database::Database::new(&db_path);
    if !db.check_path() {
        db.create_default_database().unwrap();
    }

    terminal::write("Daily ", Color::Green);
    terminal::write("Scheduler\n", Color::Blue);

    terminal::write("Write what you did ", Color::White);
    terminal::write("today", Color::Magenta);
    terminal::write(" to make your tomorrow ", Color::White);
    terminal::write("better!\n", Color::Yellow);

    // get description and hours colors
    let color_desc = config_file
        .get("colors", "description")
        .expect("could not find desc color in config file");
    let color_desc = terminal::match_term_color(&color_desc);
    let color_hours = config_file
        .get("colors", "hours")
        .expect("could not find hours color in config file");
    let color_hours = terminal::match_term_color(&color_hours);

    let color_desc = match color_desc {
        Ok(color) => color,
        Err(_) => panic!("desc color not found in config file"),
    };

    let color_hours = match color_hours {
        Ok(color) => color,
        Err(_) => panic!("hours color not found in config file"),
    };

    terminal::write("description color ", color_desc);
    terminal::write("hours color\n", color_hours);
    show_main_menu();
}
