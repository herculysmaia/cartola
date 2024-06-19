// Importando controles das sub-telas de gerencia
mod busca;
mod home;

// Importando necessidades locais
use crate::app::model::{gerencia::MsgGerencia, Tela};

// Definindo comportamento da tela de gerencia
pub fn update(screen: &mut Tela, message: MsgGerencia) {
    match message {
        MsgGerencia::MsgGerenciaHome(message) => home::update(message, screen),
        MsgGerencia::MsgBuscaTime(message) => busca::update(screen, message),
    }
}
