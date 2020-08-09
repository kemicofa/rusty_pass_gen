use rand::random;
use clap::{App, load_yaml, ArgMatches};
use std::cmp::Ordering;
const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &str = "1234567890";
const SYMBOLS: &str = "§±!@#$%^&*()_+=-[]{}|/?.>,<;:";

fn generate_characters(string: &str, len: usize) -> String {
    let mut res = String::new();
    let count = string.chars().count();
    for _ in 0..len {
        let rng = random::<usize>();
        let index= rng % count;
        res.push(string.chars().nth(index).unwrap());
    }
    res
}

fn shuffle(string: String) -> String {
    let mut shuffed_vec = string.chars().collect::<Vec<char>>();
    
    shuffed_vec.sort_by(|_,_| {
        match random::<usize>() % 2 {
            0 => Ordering::Less,
            1 => Ordering::Greater,
            _ => Ordering::Equal
        }
    });

    shuffed_vec.into_iter().collect()
}

fn parse_input <T: std::str::FromStr>(matches: &ArgMatches, name: &str) -> Option<T> {
    matches.value_of(name)?.parse().ok()
}

fn main() {

    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    let password_length: f32 = parse_input(&matches, "length").unwrap_or(20.);
    let numbers_percent: f32 = parse_input(&matches, "numbers").unwrap_or(0.25);
    let symbols_percent: f32 = parse_input(&matches, "symbols").unwrap_or(0.25);
    let letters_percent: f32 = parse_input(&matches, "letters").unwrap_or(0.5);

    if numbers_percent + symbols_percent + letters_percent > 1.0 {
        panic!("number of symbols + numeric values + letters cannot exceed 100%");
    }

    let mut password = String::with_capacity(password_length as usize);
    password += &generate_characters(NUMBERS, (numbers_percent * password_length) as _);
    password += &generate_characters(LETTERS, (letters_percent * password_length) as _);
    password += &generate_characters(SYMBOLS, (symbols_percent * password_length) as _);

    println!("{}", shuffle(password));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_characters() {
        assert_eq!(generate_characters("a", 10), "aaaaaaaaaa");
    }
}