use std::env;
use yansi::Color;

use daily_scheduler::config::Config;
use daily_scheduler::database::Database;
use daily_scheduler::terminal;
use daily_scheduler::Scheduler;

const CONFIG_FILE: &str = "config.ini";

fn help() {
    terminal::write("[Help Section]\n", Color::Cyan);
    terminal::write("today - ", Color::Green);
    terminal::write("add a task executed today\n", Color::White);
}

fn main() {
    if cfg!(windows) && !yansi::Paint::enable_windows_ascii() {
        yansi::Paint::disable();
    }

    let mut config_file = Config::new(CONFIG_FILE);
    if !config_file.check_path() && config_file.create_default_config().is_ok() {
        terminal::write("[info] created config file!\n", Color::Yellow);
    }

    if let Err(e) = config_file.load_config() {
        terminal::write(e, Color::Red);
    }

    let db_path = config_file.get("database", "path");
    if db_path.is_none() {
        panic!("database path not found in config file");
    }

    let db = Database::new(&db_path.unwrap());
    if !db.check_path() {
        if let Err(e) = db.create_default_database() {
            panic!("{}", e);
        }
    }

    // get description and hours colors
    let color_desc = config_file
        .get("colors", "description")
        .and_then(|desc| terminal::match_term_color(&desc).ok())
        .expect("could not find terminal description color in config file");

    let color_hours = config_file
        .get("colors", "hours")
        .and_then(|desc| terminal::match_term_color(&desc).ok())
        .expect("could not find terminal hours color in config file");

    let mut scheduler = Scheduler {
        config_file,
        db,
        color_desc,
        color_hours,
    };

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 {
        match &args[1][..] {
            "today" => {}
            _ => help(),
        }
    } else {
        scheduler.main_menu();
    }
}
