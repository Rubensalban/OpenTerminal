use ssh2::Session;
use std::net::TcpStream;
use std::path::Path;
use std::io::Read;
use std::sync::{Arc, Mutex};

pub struct SshSession {
    session: Session,
}

impl SshSession {
    pub fn new(
        host: &str,
        port: &str,
        username: &str,
        password: &str,
        use_key: bool,
        key_path: &str,
    ) -> Result<Self, String> {
        // Créer l'adresse complète avec le port
        let addr = format!("{}:{}", host, port);
        
        // Se connecter au serveur via TCP
        let tcp = TcpStream::connect(&addr).map_err(|e| e.to_string())?;
        
        // Créer une session SSH
        let mut sess = Session::new().map_err(|e| e.to_string())?;
        sess.set_tcp_stream(tcp);
        sess.handshake().map_err(|e| e.to_string())?;
        
        // Authentification
        if use_key && !key_path.is_empty() {
            // Authentification par clé
            sess.userauth_pubkey_file(
                username,
                None,
                Path::new(key_path),
                None,
            ).map_err(|e| e.to_string())?;
        } else {
            // Authentification par mot de passe
            sess.userauth_password(username, password)
                .map_err(|e| e.to_string())?;
        }
        
        // Vérifier si l'authentification a réussi
        if !sess.authenticated() {
            return Err(String::from("Authentification échouée"));
        }
        
        Ok(SshSession { session: sess })
    }
    
    pub fn execute_command(&self, command: &str) -> Result<String, String> {
        // Ouvrir un canal pour exécuter la commande
        let mut channel = self.session.channel_session()
            .map_err(|e| e.to_string())?;
            
        // Exécuter la commande
        channel.exec(command).map_err(|e| e.to_string())?;
        
        // Lire la sortie
        let mut output = String::new();
        channel.read_to_string(&mut output).map_err(|e| e.to_string())?;
        
        // Attendre la fin de l'exécution
        channel.wait_close().map_err(|e| e.to_string())?;
        
        Ok(output)
    }
}

// Fonction simple pour tester la connexion
pub fn test_connection(
    host: &str,
    port: &str,
    username: &str,
    password: &str,
    use_key: bool,
    key_path: &str,
) -> Result<(), String> {
    let _session = SshSession::new(host, port, username, password, use_key, key_path)?;
    Ok(())
}