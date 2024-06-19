// Importando controles das sub telas iniciais
mod gerencia;
mod home;

// Importando necessidades do Iced
use iced::Command;

// Importado necessidades locais
use crate::app::model::{MsgPrincipal, Tela};

// Implementação da atualização de tela
impl Tela {
    pub fn update(&mut self, message: MsgPrincipal) -> Command<MsgPrincipal> {
        match self {
            Tela::Home => {
                if let MsgPrincipal::MsgHome(message) = message {
                    home::update(message, self);
                };
            }
            Tela::Gerencia(_dados) => {
                if let MsgPrincipal::MsgGerencia(message) = message {
                    gerencia::update(self, message);
                };
            }
        }
        Command::none()
    }
}
