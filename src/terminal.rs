use yansi::{Color, Paint};
use std::io::prelude::*;

pub fn ask_input() -> String {
    let mut response = String::new();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut response).unwrap();
    response
}

pub fn write(text: &str, color: Color) {
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

pub fn match_term_color(color: &str) -> Result<Color, &str> {
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
