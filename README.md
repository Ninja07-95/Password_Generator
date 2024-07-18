# Password Generator

## Description

Password Generator is a command-line tool that generates strong, customizable passwords or passphrases. You can specify various options such as length, inclusion of numbers, symbols, uppercase, and lowercase letters, as well as the number of passwords to generate. Additionally, the generated passwords can be saved in JSON, CSV, or TXT format.

## Features

- Generate strong passwords or passphrases
- Customize password length and character inclusion
- Generate multiple passwords at once
- Save generated passwords in JSON, CSV, or TXT format
- Display generated passwords in color in the terminal

## Usage

To use the Password Generator, run the following command with the desired options:

```sh
passgen [OPTIONS]

Options

-nn | --no-nums : No numbers
-ns | --no-symbols : No symbols
-nl | --no-lowercase : No lowercase letters
-nu | --no-uppercase : No uppercase letters
-l | --len <len> : Specify password length
-n | --num <num> : Number of passwords to generate
-p | --passphrase : Generate passphrases instead of passwords
-w | --words <words> : Number of words in passphrase (default: 4)
-h | --help : Display this message
-o | --out <file> : Save to file
-f | --format <format> : Specify file format (json, csv, txt)
```

# Example

Generate 5 passwords of length 12 without symbols and save them in a file named passwords.json:

```sh
passgen -n 5 -l 12 -ns -o passwords.json -f json
```

Generate a passphrase with 6 words (using the wordlist.txt in src/):

```sh
passgen -p -w 6
```

# Installation

Clone the repository and build the project using Cargo:

```sh
git clone https://github.com/yourusername/Password_Generator.git
cd Password_Generator
cargo build --release
```

# Running Tests

To run the tests, use the following command:

```sh
cargo test
```

# Dependencies

- rand: For random number generation
- serde: For serialization and deserialization of JSON
- csv: For writing CSV files
- termcolor: For colored terminal output
- simplelog: For logging

# Authors

BARRIZ Anass
Samba
NGUIDIA Arthur

