use rand::random;

const LETTERS: &str = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
const NUMBERS: &str = "1234567890";
const SYMBOLS: &str = "!@#$%^&*()_+=-[]{}|/?.>,<;:";

/// Generates a String of randomely ordered characters
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

/// Shuffles a given string randomely
fn shuffle(string: String) -> String {
    let mut chars = string.chars().collect::<Vec<_>>();
    let count = string.chars().count();

    for i in 0..count {
        chars.swap(i, random::<usize>() % count);
    }

    chars.into_iter().collect::<String>()
}

#[export_name = "generate_password"]
pub fn generate_password(password_length: u32, letters_percent: f32, numbers_percent: f32, symbols_percent: f32) -> String {
    let mut password = String::with_capacity(password_length as usize);
    password += &generate_characters(NUMBERS, (numbers_percent * password_length as f32) as _);
    password += &generate_characters(LETTERS, (letters_percent * password_length as f32) as _);
    password += &generate_characters(SYMBOLS, (symbols_percent * password_length as f32) as _);
    shuffle(password)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_characters() {
        assert_eq!(generate_characters("a", 10), "aaaaaaaaaa");
    }
}