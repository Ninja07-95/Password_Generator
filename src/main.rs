mod utils; // Importe le module utils

use utils::{create_pass, create_passphrase, PassConfig, save_to_json, save_to_csv, save_to_txt}; // Importe les fonctions et structs nécessaires depuis utils
use std::{env, io::Write}; // Importe des modules de la bibliothèque standard
use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor}; // Pour la coloration de la sortie terminal
use simplelog::{Config, LevelFilter, SimpleLogger}; // Pour la gestion des logs
use log::info; // Pour la journalisation

fn main() {
    // Initialise le logger
    SimpleLogger::init(LevelFilter::Info, Config::default()).unwrap();

    // Récupère les arguments de la ligne de commande
    let config = env::args().collect::<Vec<String>>();
    let mut has_nums = true;
    let mut has_symbols = true;
    let mut has_lowercase = true;
    let mut has_uppercase = true;
    let mut length = 10;
    let mut num_passwords = 1;
    let mut filename = None;
    let mut file_format = "json";
    let mut use_passphrase = false;
    let mut passphrase_words = 4;
    
    // Message d'aide
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
        -f  | --format <format> Specify file format (json, csv, txt)
    ";

    // Variable pour stocker une option invalide
    let mut invalid_option = None;

    // Boucle sur les arguments et gère les options
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
                if let Some(next_arg) = config.get(index + 1) {
                    filename = Some(next_arg.clone());
                }
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
            "--format" | "-f" => {
                file_format = match config.get(index + 1) {
                    Some(format) if format == "json" || format == "csv" || format == "txt" => format,
                    _ => {
                        eprintln!("Error: Invalid format '{}'\n", config.get(index + 1).unwrap_or(&String::new()));
                        println!("{}", help_msg);
                        std::process::exit(1);
                    }
                };
            }
            "--help" | "-h" => {
                println!("{}", help_msg);
                std::process::exit(0);
            }
            _ => {
                if arg.starts_with('-') {
                    invalid_option = Some(arg.clone());
                    break;
                }
            }
        };
    }

    // Gère les options invalides
    if let Some(option) = invalid_option {
        eprintln!("Error: Invalid option '{}'\n", option);
        println!("{}", help_msg);
        std::process::exit(1);
    }

    // Crée la configuration de génération de mot de passe
    let pass_config = PassConfig {
        length,
        has_nums,
        has_symbols,
        has_lowercase,
        has_uppercase,
        filename: filename.clone(),
    };

    // Initialise la sortie standard avec coloration
    let mut stdout = StandardStream::stdout(ColorChoice::Always);
    let mut passwords = Vec::new();

    // Génère les mots de passe ou les phrases de passe
    if use_passphrase {
        info!("Generating passphrases");
        for _ in 0..num_passwords {
            let passphrase = create_passphrase(passphrase_words);
            passwords.push(passphrase);
        }
    } else {
        info!("Generating passwords");
        for _ in 0..num_passwords {
            let password = create_pass(&pass_config);
            passwords.push(password);
        }
    }

    // Affiche les mots de passe ou les phrases de passe générés
    stdout.set_color(ColorSpec::new().set_fg(Some(Color::Yellow)).set_bold(true)).unwrap();
    writeln!(stdout, "Generated {}: ", if use_passphrase { "Passphrases" } else { "Passwords" }).unwrap();

    for password in &passwords {
        stdout.set_color(ColorSpec::new().set_fg(Some(Color::Rgb(0, 255, 0))).set_bold(true)).unwrap();
        writeln!(stdout, "{}", password).unwrap();
    }

    stdout.reset().unwrap();

    // Sauvegarde les mots de passe ou les phrases de passe dans un fichier si spécifié
    if let Some(filename) = filename {
        match file_format {
            "json" => {
                if let Err(e) = save_to_json(&filename, &passwords) {
                    eprintln!("Error saving passwords to JSON file: {}", e);
                } else {
                    info!("Passwords saved to JSON file '{}'", filename);
                }
            },
            "csv" => {
                if let Err(e) = save_to_csv(&filename, &passwords) {
                    eprintln!("Error saving passwords to CSV file: {}", e);
                } else {
                    info!("Passwords saved to CSV file '{}'", filename);
                }
            },
            "txt" => {
                if let Err(e) = save_to_txt(&filename, &passwords) {
                    eprintln!("Error saving passwords to TXT file: {}", e);
                } else {
                    info!("Passwords saved to TXT file '{}'", filename);
                }
            },
            _ => unreachable!(),
        }
    }
}
