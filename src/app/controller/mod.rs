// Importando controles das sub telas iniciais
mod db;
mod gerencia;
mod home;
mod request;

// Importando necessidades do Iced
use iced::Command;

// Importado necessidades locais
use crate::app::model::{MsgPrincipal, Tela};

// Implementação da atualização de tela
impl Tela {
    pub fn update(&mut self, message: MsgPrincipal) -> Command<MsgPrincipal> {
        let mut retorno = Command::none();

        // Definição de tela a ser exibida
        match self {
            Tela::Home => {
                if let MsgPrincipal::MsgHome(message) = message {
                    retorno = home::update(self, message)
                };
            }
            Tela::Gerencia(_dados) => {
                if let MsgPrincipal::MsgGerencia(message) = message {
                    retorno = gerencia::update(self, message)
                };
            }
        }
        retorno
    }
}
