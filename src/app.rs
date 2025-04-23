use crate::ssh::{self, SshSession};
use iced::{Alignment, Element, Length, Sandbox};
use iced::widget::{Button, Checkbox, Column, Container, Row, Text, TextInput, scrollable, Scrollable};


// Structure pour stocker les informations de connexion
pub struct SshConnection {
    host: String,
    port: String,
    username: String,
    password: String,
    use_key: bool,
    key_path: String,
    status: String,
    connected: bool,
    terminal_input: String,
    terminal_output: Vec<String>,
    ssh_session: Option<SshSession>,
}

// Enum pour les messages de l'interface utilisateur
#[derive(Debug, Clone)]
pub enum Message {
    HostChanged(String),
    PortChanged(String),
    UsernameChanged(String),
    PasswordChanged(String),
    UseKeyToggled(bool),
    KeyPathChanged(String),
    Connect,
    Disconnect,
    TerminalInputChanged(String),
    ExecuteCommand,
}

impl Sandbox for SshConnection {
    type Message = Message;

    fn new() -> Self {
        SshConnection {
            host: String::from(""),
            port: String::from("22"),
            username: String::from(""),
            password: String::from(""),
            use_key: false,
            key_path: String::from(""),
            status: String::from("Prêt à se connecter"),
            connected: false,
            terminal_input: String::from(""),
            terminal_output: Vec::new(),
            ssh_session: None,
        }
    }

    fn title(&self) -> String {
        String::from("Client SSH Rust")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::HostChanged(value) => self.host = value,
            Message::PortChanged(value) => self.port = value,
            Message::UsernameChanged(value) => self.username = value,
            Message::PasswordChanged(value) => self.password = value,
            Message::UseKeyToggled(value) => self.use_key = value,
            Message::KeyPathChanged(value) => self.key_path = value,
            Message::Connect => {
                self.status = String::from("Tentative de connexion...");
                
                // Tenter la connexion SSH
                match ssh::SshSession::new(
                    &self.host,
                    &self.port,
                    &self.username,
                    &self.password,
                    self.use_key,
                    &self.key_path,
                ) {
                    Ok(session) => {
                        self.status = String::from("Connexion réussie!");
                        self.connected = true;
                        self.ssh_session = Some(session);
                        self.terminal_output.push(format!("Connecté à {}:{}", self.host, self.port));
                    },
                    Err(e) => {
                        self.status = format!("Erreur: {}", e);
                        self.connected = false;
                    }
                }
            },
            Message::Disconnect => {
                self.connected = false;
                self.ssh_session = None;
                self.status = String::from("Déconnecté");
                self.terminal_output.push(String::from("Déconnecté du serveur"));
            },
            Message::TerminalInputChanged(value) => {
                self.terminal_input = value;
            },
            Message::ExecuteCommand => {
                if let Some(ref session) = self.ssh_session {
                    let command = self.terminal_input.clone();
                    self.terminal_output.push(format!("$ {}", command));
                    
                    match session.execute_command(&command) {
                        Ok(output) => {
                            self.terminal_output.push(output);
                        },
                        Err(e) => {
                            self.terminal_output.push(format!("Erreur: {}", e));
                        }
                    }
                    
                    self.terminal_input = String::from("");
                }
            }
        }
    }

    fn view(&self) -> Element<Message> {
        if !self.connected {
            // Affichage du formulaire de connexion
            let host_input = TextInput::new(
                "Adresse du serveur",
                &self.host,
            )
            .on_input(Message::HostChanged);
            
            let port_input = TextInput::new(
                "Port",
                &self.port,
            )
            .on_input(Message::PortChanged);
            
            let username_input = TextInput::new(
                "Nom d'utilisateur",
                &self.username,
            )
            .on_input(Message::UsernameChanged);
            
            let password_input = TextInput::new(
                "Mot de passe",
                &self.password,
            )
            .password()
            .on_input(Message::PasswordChanged);

            let key_checkbox = Checkbox::new(
                "Utiliser une clé SSH",
                self.use_key,
                Message::UseKeyToggled,
            );

            let key_path_input = TextInput::new(
                "Chemin de la clé",
                &self.key_path,
            )
            .on_input(Message::KeyPathChanged);
            
            let connect_button = Button::new(
                Text::new("Connecter"),
            )
            .on_press(Message::Connect);
            
            let status_text = Text::new(&self.status);
            
            let content = Column::new()
                .padding(20)
                .spacing(20)
                .max_width(400)
                .align_items(Alignment::Center)
                .push(Text::new("Client SSH Rust").size(24))
                .push(host_input)
                .push(port_input)
                .push(username_input)
                .push(password_input)
                .push(key_checkbox)
                .push(key_path_input)
                .push(connect_button)
                .push(status_text);
                
            Container::new(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .center_x()
                .center_y()
                .into()
        } else {
            // Affichage du terminal
            let terminal_output = self.terminal_output.iter().fold(
                Column::new().spacing(10).padding(10),
                |column, line| {
                    column.push(Text::new(line))
                }
            );
            
            let terminal_scroll = Scrollable::new(terminal_output)
                .height(Length::FillPortion(8))
                .width(Length::Fill);
            
            let input_row = Row::new()
                .spacing(10)
                .push(
                    TextInput::new(
                        "Entrez une commande",
                        &self.terminal_input,
                    )
                    .on_input(Message::TerminalInputChanged)
                    .on_submit(Message::ExecuteCommand)
                    .width(Length::Fill)
                )
                .push(
                    Button::new(Text::new("Exécuter"))
                        .on_press(Message::ExecuteCommand)
                );
            
            let disconnect_button = Button::new(Text::new("Déconnecter"))
                .on_press(Message::Disconnect);
            
            let content = Column::new()
                .padding(20)
                .spacing(20)
                .width(Length::Fill)
                .height(Length::Fill)
                .push(Text::new(format!("Terminal SSH - Connecté à {}:{}", self.host, self.port)).size(24))
                .push(terminal_scroll)
                .push(input_row)
                .push(disconnect_button);
                
            Container::new(content)
                .width(Length::Fill)
                .height(Length::Fill)
                .into()
        }
    }
}