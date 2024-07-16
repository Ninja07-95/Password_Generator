mod utils;

use utils::{create_pass, create_passphrase, PassConfig, save_to_file};
use std::{env, io::Write};
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use simplelog::{Config, LevelFilter, SimpleLogger};

fn main() {
    SimpleLogger::init(LevelFilter::Info, Config::default()).unwrap();

    let config = env::args().collect::<Vec<String>>();
    let mut has_nums = true;
    let mut has_symbols = true;
    let mut has_lowercase = true;
    let mut has_uppercase = true;
    let mut length = 10;
    let mut num_passwords = 1;
    let mut filename = None;
    let mut use_passphrase = false;
    let mut passphrase_words = 4;
    let help_msg = "passgen [OPTIONS]

    OPTIONS:
        -nn | --no-nums         No numbers
        -ns | --no-symbols      No symbols
        -nl | --no-lowercase    No lowercase letters
        -nu | --no-uppercase    No uppercase letters
        -l  | --len <len>       Specify password length
        -n  | --num <num>       Number of passwords to generate
        -p  | --passphrase      Generate passphrases instead of passwords
        -w  | --words <words>   Number of words in passphrase (default: 4)
        -h  | --help            Display this message
        -o  | --out <file>      Save to file
    ";

    let mut invalid_option = None;

    for (index, arg) in config.iter().enumerate() {
        match &arg[..] {
            "--no-nums" | "-nn" => {
                has_nums = false;
            }
            "--no-symbols" | "-ns" => {
                has_symbols = false;
            }
            "--no-lowercase" | "-nl" => {
                has_lowercase = false;
            }
            "--no-uppercase" | "-nu" => {
                has_uppercase = false;
            }
            "--out" | "-o" => {
                filename = Some(config[index + 1].to_string());
            }
            "--len" | "-l" => {
                length = match config.get(index + 1) {
                    Some(len) => match len.parse::<usize>() {
                        Ok(l) if l > 0 => l,
                        _ => {
                            eprintln!("Error: Invalid length '{}'\n", len);
                            println!("{}", help_msg);
                            std::process::exit(1);
                        }
                    },
                    None => {
                        eprintln!("Error: No length provided\n");
                        println!("{}", help_msg);
                        std::process::exit(1);
                    }
                };
            }
            "--num" | "-n" => {
                num_passwords = match config.get(index + 1) {
                    Some(num) => match num.parse::<usize>() {
                        Ok(n) if n > 0 => n,
                        _ => {
                            eprintln!("Error: Invalid number of passwords '{}'\n", num);
                            println!("{}", help_msg);
                            std::process::exit(1);
                        }
                    },
                    None => {
                        eprintln!("Error: No number of passwords provided\n");
                        println!("{}", help_msg);
                        std::process::exit(1);
                    }
                };
            }
            "--passphrase" | "-p" => {
                use_passphrase = true;
            }
            "--words" | "-w" => {
                passphrase_words = match config.get(index + 1) {
                    Some(words) => match words.parse::<usize>() {
                        Ok(w) if w > 0 => w,
                        _ => {
                            eprintln!("Error: Invalid number of words '{}'\n", words);
                            println!("{}", help_msg);
                            std::process::exit(1);
                        }
                    },
                    None => {
                        eprintln!("Error: No number of words provided\n");
                        println!("{}", help_msg);
                        std::process::exit(1);
                    }
                };
            }
            "--help" | "-h" => {
                println!("{}", help_msg);
                std::process::exit(0);
            }
            _ if arg.starts_with('-') => {
                invalid_option = Some(arg.to_string());
                break;
            }
            _ => continue,
        };
    }

    if let Some(invalid_option) = invalid_option {
        eprintln!("Error: Invalid option '{}'\n", invalid_option);
        println!("{}", help_msg);
        std::process::exit(1);
    }

    let pass_config = PassConfig {
        length,
        has_nums,
        has_symbols,
        has_lowercase,
        has_uppercase,
        filename: filename.clone(),
    };

    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut passwords = Vec::new();

    if use_passphrase {
        log::info!("Generating passphrases");
        for _ in 0..num_passwords {
            let passphrase = create_passphrase(passphrase_words);
            passwords.push(passphrase);
        }
    } else {
        log::info!("Generating passwords");
        for _ in 0..num_passwords {
            let password = create_pass(&pass_config);
            passwords.push(password);
        }
    }

    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)).set_bold(true)).unwrap();
    writeln!(stdout, "Generated {}: ", if use_passphrase { "Passphrases" } else { "Passwords" }).unwrap();

    for password in &passwords {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(255, 165, 0))).set_bold(false)).unwrap();
        writeln!(stdout, "{}", password).unwrap();
    }

    if let Some(filename) = filename {
        if let Err(e) = save_to_file(&filename, &passwords) {
            eprintln!("Error saving passwords to file: {}", e);
        } else {
            log::info!("Passwords saved to file '{}'", filename);
        }
    }
}
