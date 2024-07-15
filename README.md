# Password_Generator
Générateur des mots de passes simple en Rust avec des options de configuration.

Description : 

Ce projet est un générateur de mots de passe configurable en ligne de commande. Vous pouvez spécifier des options telles que l'inclusion de chiffres, de symboles, la longueur du mot de passe et le fichier de sortie.


Installation : 


git clone https://github.com/Ninja07-95/Password_Generator.git
cd password-generator

Utilisation :


Compilez le projet avec cargo build puis exécutez avec cargo run et les options désirées.



Options disponibles :


-nn, --no-nums : Exclut les chiffres du mot de passe.
-ns, --no-symbols : Exclut les symboles du mot de passe.
-l <len>, --len <len> : Spécifie la longueur du mot de passe.
-h, --help : Affiche le message d'aide.
-o <file>, --out <file> : Enregistre le mot de passe dans un fichier.



Exemples : 

Générer un mot de passe avec les options par défaut :

cargo run


Générer un mot de passe de 12 caractères sans chiffres :


cargo run -- --no-nums --len 12
