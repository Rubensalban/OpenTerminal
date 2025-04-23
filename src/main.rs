mod app;
mod ssh;

use iced::{Settings, Sandbox};

fn main() -> iced::Result {
    app::SshConnection::run(Settings::default())
}