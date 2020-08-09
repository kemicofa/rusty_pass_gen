use rand::random;
use clap::{App, load_yaml};
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

fn main() {

    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    let password_length: f32 = matches.value_of("length").unwrap_or("20").parse().unwrap();
    let numbers_percent: f32 = matches.value_of("numbers").unwrap_or("0.25").parse().unwrap();
    let symbols_percent: f32 = matches.value_of("symbols").unwrap_or("0.25").parse().unwrap();
    let letters_percent: f32 = matches.value_of("letters").unwrap_or("0.5").parse().unwrap();

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