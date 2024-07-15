mod utils;

use utils::{create_pass, PassConfig};
use std::{env, io::Write};

fn main() {
    let config = env::args().collect::<Vec<String>>();
    let mut has_nums = true;
    let mut has_symbols = true;
    let mut length = 10;
    let mut filename = None;
    let help_msg = "passgen [OPTIONS]

    OPTIONS:
        -nn | --no-nums         No numbers
        -ns | -no-symbols       No symbols
        -l  | --len <len>       Specify password length
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
            "--out" | "-o" => {
                filename = Some(config[index + 1].to_string());
            }
            "--len" | "-l" => {
                length = config[index + 1].parse().unwrap();
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

    let password = create_pass(PassConfig {
        length,
        has_nums,
        has_symbols,
        filename,
    });

    writeln!(std::io::stdout(), "Generated Password:").unwrap();
    writeln!(std::io::stdout(), "{}", password).unwrap();
}
