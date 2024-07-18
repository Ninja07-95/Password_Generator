use rand::Rng; // Pour la génération aléatoire
use serde::{Serialize, Deserialize}; // Pour la sérialisation et désérialisation JSON
use std::fs::File; // Pour la gestion des fichiers
use std::io::{Write, BufWriter}; // Pour l'écriture dans les fichiers
use std::error::Error; // Pour la gestion des erreurs
use csv::Writer; // Pour l'écriture dans des fichiers CSV

// Structure de configuration pour la génération de mots de passe
#[derive(Debug, Serialize, Deserialize)]
pub struct PassConfig {
    pub length: usize, // Longueur du mot de passe
    pub has_nums: bool, // Inclure des chiffres
    pub has_symbols: bool, // Inclure des symboles
    pub has_lowercase: bool, // Inclure des lettres minuscules
    pub has_uppercase: bool, // Inclure des lettres majuscules
    pub filename: Option<String>, // Nom du fichier de sortie
}

// Fonction pour créer un mot de passe basé sur la configuration
pub fn create_pass(config: &PassConfig) -> String {
    let mut rng = rand::thread_rng(); // Initialisation du générateur de nombres aléatoires
    let mut password = String::with_capacity(config.length); // Création d'une chaîne de caractères de la longueur spécifiée
    let chars = [
        ('0'..='9').collect::<Vec<_>>(), // Chiffres
        ('a'..='z').collect::<Vec<_>>(), // Lettres minuscules
        ('A'..='Z').collect::<Vec<_>>(), // Lettres majuscules
        ('!'..='/').chain(':'..='@').chain('['..='`').chain('{'..='~').collect::<Vec<_>>(), // Symboles
    ];

    let mut selected_chars = Vec::new(); // Caractères sélectionnés pour le mot de passe

    // Ajoute les caractères appropriés en fonction de la configuration
    if config.has_nums {
        selected_chars.extend(&chars[0]);
    }
    if config.has_lowercase {
        selected_chars.extend(&chars[1]);
    }
    if config.has_uppercase {
        selected_chars.extend(&chars[2]);
    }
    if config.has_symbols {
        selected_chars.extend(&chars[3]);
    }

    // Génère le mot de passe
    for _ in 0..config.length {
        let idx = rng.gen_range(0..selected_chars.len());
        password.push(selected_chars[idx]);
    }

    password
}

// Fonction pour créer une phrase de passe
pub fn create_passphrase(num_words: usize) -> String {
    let words = include_str!("wordlist.txt").lines().collect::<Vec<_>>(); // Charge une liste de mots
    let mut rng = rand::thread_rng(); // Initialisation du générateur de nombres aléatoires
    let mut passphrase = Vec::with_capacity(num_words); // Création d'un vecteur pour les mots de la phrase de passe

    // Sélectionne des mots aléatoires pour la phrase de passe
    for _ in 0..num_words {
        let idx = rng.gen_range(0..words.len());
        passphrase.push(words[idx]);
    }

    passphrase.join(" ") // Joint les mots avec des espaces
}

// Fonction pour sauvegarder les mots de passe dans un fichier JSON
pub fn save_to_json(filename: &str, passwords: &[String]) -> Result<(), Box<dyn Error>> {
    let file = File::create(filename)?; // Crée un fichier
    serde_json::to_writer_pretty(file, &passwords)?; // Écrit les mots de passe dans le fichier en format JSON
    Ok(())
}

// Fonction pour sauvegarder les mots de passe dans un fichier CSV
pub fn save_to_csv(filename: &str, passwords: &[String]) -> Result<(), Box<dyn Error>> {
    let file = File::create(filename)?; // Crée un fichier
    let mut wtr = Writer::from_writer(file); // Initialisation de l'écrivain CSV
    
    // Écrit chaque mot de passe dans le fichier CSV
    for password in passwords {
        wtr.write_record(&[password])?;
    }
    
    wtr.flush()?; // Vide le buffer
    Ok(())
}

// Fonction pour sauvegarder les mots de passe dans un fichier TXT
pub fn save_to_txt(filename: &str, passwords: &[String]) -> Result<(), Box<dyn Error>> {
    let file = File::create(filename)?; // Crée un fichier
    let mut writer = BufWriter::new(file); // Initialisation de l'écrivain de buffer
    
    // Écrit chaque mot de passe dans le fichier TXT
    for password in passwords {
        writeln!(writer, "{}", password)?;
    }
    
    writer.flush()?; // Vide le buffer
    Ok(())
}
