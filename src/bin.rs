//! # Rusty Pass Gen

use clap::{App, load_yaml, ArgMatches};
use mylib::generate_password;

/// Parses command line input from Clap
fn parse_input <T: std::str::FromStr>(matches: &ArgMatches, name: &str) -> Option<T> {
    matches.value_of(name)?.parse().ok()
}

fn main() {

    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    let password_length: u32 = parse_input(&matches, "length").unwrap_or(20);
    let numbers_percent: f32 = parse_input(&matches, "numbers").unwrap_or(0.25);
    let symbols_percent: f32 = parse_input(&matches, "symbols").unwrap_or(0.25);
    let letters_percent: f32 = parse_input(&matches, "letters").unwrap_or(0.5);

    if numbers_percent + symbols_percent + letters_percent > 1.0 {
        panic!("number of symbols + numeric values + letters cannot exceed 100%");
    }

    println!("{}", generate_password(password_length, letters_percent, numbers_percent, symbols_percent));
}

