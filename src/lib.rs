pub mod config;
pub mod database;
pub mod terminal;

use config::Config;
use database::Database;

use yansi::Color;

pub struct Scheduler {
    pub config_file: Config,
    pub db: Database,
    pub color_desc: Color,
    pub color_hours: Color,
}

impl Scheduler {
    fn show_selection(&self) {
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

    pub fn main_menu(&mut self) {
        terminal::write("Daily ", Color::Green);
        terminal::write("Scheduler\n", Color::Blue);

        terminal::write("Write what you did ", Color::White);
        terminal::write("today", Color::Magenta);
        terminal::write(" to make your tomorrow ", Color::White);
        terminal::write("better!\n", Color::Yellow);

        self.show_selection();
        self.db.insert_test();
    }
}
