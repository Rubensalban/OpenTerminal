# SSH Client Rust

Un client SSH simple et moderne écrit en Rust avec une interface graphique utilisant Iced.

## À propos

SSH Client Rust est une application de bureau légère qui permet de se connecter à des serveurs distants via SSH. Inspiré par des applications comme Termius, ce client vise à offrir une expérience utilisateur moderne tout en se concentrant sur les fonctionnalités essentielles de SSH.

## Fonctionnalités

- Interface utilisateur graphique intuitive
- Connexion SSH par mot de passe ou clé privée
- Terminal interactif pour exécuter des commandes à distance
- Affichage des résultats des commandes
- Interface simple et légère

## Captures d'écran


## Technologies utilisées

- [Rust](https://www.rust-lang.org/) - Un langage de programmation axé sur la performance et la sûreté
- [Iced](https://github.com/iced-rs/iced) - Une bibliothèque pour créer des interfaces graphiques multiplateformes en Rust
- [ssh2-rs](https://github.com/alexcrichton/ssh2-rs) - Bindings Rust pour la bibliothèque libssh2

## Prérequis

- Rust (édition 2021 ou supérieure)
- Bibliothèques de développement pour OpenSSL et libssh2:
  - Ubuntu/Debian: `sudo apt install libssl-dev libssh2-1-dev pkg-config`
  - macOS (avec Homebrew): `brew install openssl libssh2`
  - Windows: Installation via MSYS2 ou vcpkg

## Installation

1. Clonez ce dépôt:
   ```
   git clone https://github.com/votre-nom/ssh-client-rust.git
   cd ssh-client-rust
   ```

2. Compilez le projet:
   ```
   cargo build --release
   ```

3. Exécutez l'application:
   ```
   cargo run --release
   ```

## Utilisation

1. Lancez l'application
2. Entrez les informations de connexion:
   - Adresse du serveur
   - Port (par défaut: 22)
   - Nom d'utilisateur
   - Mot de passe ou chemin vers votre clé privée
3. Cliquez sur "Connecter"
4. Une fois connecté, vous pouvez entrer des commandes dans le terminal interactif

## Contribuer

Les contributions sont les bienvenues!

### Idées de contributions

- Amélioration de l'interface utilisateur
- Ajout de la gestion de plusieurs connexions
- Support pour le transfert de fichiers (SFTP)
- Sauvegarde des connexions précédentes
- Support pour les tunnels SSH (port forwarding)

## Licence

Ce projet est sous licence MIT - voir le fichier [LICENSE](LICENSE) pour plus de détails.

## Remerciements

- L'équipe Rust pour un langage de programmation incroyable
- Les développeurs d'Iced pour la bibliothèque d'interface utilisateur
- Les développeurs de ssh2-rs pour les bindings Rust vers libssh2