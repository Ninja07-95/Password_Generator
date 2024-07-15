use rand::Rng;
use std::fs::File;
use std::io::{self, Write};

pub struct PassConfig {
    pub length: usize,
    pub has_nums: bool,
    pub has_symbols: bool,
    pub filename: Option<String>,
}

pub fn create_pass(config: PassConfig) -> String {
    let mut rng = rand::thread_rng();
    let mut characters: Vec<char> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();

    if config.has_nums {
        characters.extend("0123456789".chars());
    }
    if config.has_symbols {
        characters.extend("!@#$%^&*()_+-=[]{}|;:'\",.<>?/`~".chars());
    }

    let password: String = (0..config.length)
        .map(|_| characters[rng.gen_range(0..characters.len())])
        .collect();

    if let Some(filename) = config.filename {
        save_to_file(&filename, &password).expect("Failed to save the password to file");
    }

    password
}

fn save_to_file(filename: &str, password: &str) -> io::Result<()> {
    let mut file = File::create(filename)?;
    writeln!(file, "{}", password)?;
    Ok(())
}
