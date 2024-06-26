// Importando controles das sub-telas de gerencia
mod busca;
mod home;

// Importando necessidades do Iced
use iced::Command;

// Importando necessidades locais
use crate::app::model::{gerencia::MsgGerencia, MsgPrincipal, Tela};

// Definindo comportamento da tela de gerencia
pub fn update(screen: &mut Tela, message: MsgGerencia) -> Command<MsgPrincipal> {
    match message {
        MsgGerencia::MsgGerenciaHome(message) => home::update(screen, message),
        MsgGerencia::MsgBuscaTime(message) => busca::update(screen, message),
    }
}
