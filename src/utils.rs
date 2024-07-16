use rand::Rng;
use std::fs::File;
use std::io::{self, Write};

pub struct PassConfig {
    pub length: usize,
    pub has_nums: bool,
    pub has_symbols: bool,
    pub has_lowercase: bool,
    pub has_uppercase: bool,
    pub filename: Option<String>,
}

pub fn create_pass(config: &PassConfig) -> String {
    let mut rng = rand::thread_rng();
    let mut characters = Vec::new();

    if config.has_lowercase {
        characters.extend("abcdefghijklmnopqrstuvwxyz".chars());
    }
    if config.has_uppercase {
        characters.extend("ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars());
    }
    if config.has_nums {
        characters.extend("0123456789".chars());
    }
    if config.has_symbols {
        characters.extend("!@#$%^&*()_+-=[]{}|;:'\",.<>?/`~".chars());
    }

    if characters.is_empty() {
        panic!("No character sets selected. Enable at least one character set.");
    }

    (0..config.length)
        .map(|_| characters[rng.gen_range(0..characters.len())])
        .collect()
}

pub fn create_passphrase(num_words: usize) -> String {
    let words = vec![
        "123456", "12345","123456789","password","iloveyou","princess","1234567","12345678","abc123","nicole","daniel","babygirl","monkey","lovely","jessica","654321","michael","ashley","qwerty","111111","iloveu","000000","michelle","tigger","sunshine","chocolate","apple", "banana", "cherry", "date", "elderberry", "fig", "grape", "honeydew",
        "indigo", "jackfruit", "kiwi", "lemon", "mango", "nectarine", "orange", "papaya",
        "quince", "raspberry", "strawberry", "tangerine", "ugli", "vanilla", "watermelon",
        "xigua", "yellowfruit", "zucchini",
    ];

    let mut rng = rand::thread_rng();
    (0..num_words)
        .map(|_| words[rng.gen_range(0..words.len())].to_string())
        .collect::<Vec<_>>()
        .join(" ")
}

pub fn save_to_file(filename: &str, passwords: &[String]) -> io::Result<()> {
    let mut file = File::create(filename)?;
    for password in passwords {
        writeln!(file, "{}", password)?;
    }
    Ok(())
}
