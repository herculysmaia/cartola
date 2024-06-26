// importando necessidades do Iced
use iced::Command;

// Importando necessidades locais
use crate::app::model::{gerencia::TelaGerencia, home::MsgHome, MsgPrincipal, Tela};

// definindo as ações da tela inicial
pub fn update(screen: &mut Tela, message: MsgHome) -> Command<MsgPrincipal> {
    match message {
        MsgHome::MudarParaGerencia => *screen = Tela::Gerencia(TelaGerencia::Home),
    }

    Command::none()
}
