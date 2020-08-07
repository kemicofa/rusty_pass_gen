use std::env;
use rand::random;

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyz1234567890!@#$%^&*()_+[]{}|:;?<>.";
const DEFAULT_PASSWORD_LENGTH: usize = 20;

fn get_password_length() -> usize {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        return DEFAULT_PASSWORD_LENGTH;
    }
    args[1].parse().unwrap_or(DEFAULT_PASSWORD_LENGTH)
}

fn main() {
    let password_length = get_password_length();
    let mut password = String::with_capacity(password_length);
    for _ in 0..password_length {
        let rng = random::<usize>();
        let reduced= rng % LETTERS.len();
        password.push(LETTERS.chars().nth(reduced).unwrap());
    }
    println!("{}", password);
}
