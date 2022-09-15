use yansi::Color;

use daily_scheduler::config::Config;
use daily_scheduler::database::Database;
use daily_scheduler::terminal;

const CONFIG_FILE: &str = "config.ini";

fn main() {
    if cfg!(windows) && !yansi::Paint::enable_windows_ascii() {
        yansi::Paint::disable();
    }

    let mut config_file = Config::new(CONFIG_FILE);
    if !config_file.check_path() {
        terminal::write(
            "[error] config file not found, making a new one...\n",
            Color::Red,
        );

        if config_file.create_default_config().is_ok() {
            terminal::write("[info] created config file!\n", Color::Yellow);
        }
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

    daily_scheduler::main_menu(&config_file, &db);
}
