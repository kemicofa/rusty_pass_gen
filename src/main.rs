use rand::random;
use clap::{App, load_yaml};

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &str = "1234567890";
const SYMBOLS: &str = "§±!@#$%^&*()_+=-[]{}|/?.>,<;:";

fn main() {

    let yaml = load_yaml!("cli.yml");
    let matches = App::from(yaml).get_matches();

    let password_length = matches.value_of("length").unwrap().parse().unwrap();

    let numbers: f32 = 0.1; // matches.value_of("numbers").unwrap().parse().unwrap();
    let symbols: f32 = 0.1; // matches.value_of("symbols").unwrap().parse().unwrap();
    let letters: f32 = 0.1; // matches.value_of("letters").unwrap().parse().unwrap();


    let mut password = String::with_capacity(password_length);
    for _ in 0..(password_length as f32 * numbers) as i32 {
        let rng = random::<usize>();
        let reduced= rng % LETTERS.len();
        password.push(LETTERS.chars().nth(reduced).unwrap());
    }
    for _ in 0..(password_length as f32 * symbols) as i32 {
        let rng = random::<usize>();
        let reduced= rng % NUMBERS.len();
        password.push(NUMBERS.chars().nth(reduced).unwrap());
    }
    for _ in 0..(password_length as f32 * letters) as i32 {
        let rng = random::<usize>();
        let reduced= rng % SYMBOLS.len();
        password.push(SYMBOLS.chars().nth(reduced).unwrap());
    }
    println!("{}", password);


}
