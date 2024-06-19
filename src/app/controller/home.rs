// Importando necessidades locais
use crate::app::model::{gerencia::TelaGerencia, home::MsgHome, Tela};

// definindo as ações da tela inicial
pub fn update(message: MsgHome, screen: &mut Tela) {
    match message {
        MsgHome::MudarParaGerencia => *screen = Tela::Gerencia(TelaGerencia::Home),
    }
}
