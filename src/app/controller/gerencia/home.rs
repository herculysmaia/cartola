// Importando necessidades do Iced
use iced::Command;

// Importando necessidades locais
use crate::app::model::{
    gerencia::{busca::DataGerencia, home::MsgGerenciaHome, TelaGerencia},
    MsgPrincipal, Tela,
};

// Definindo comportamento da tela inicial de gerencia
pub fn update(screen: &mut Tela, message: MsgGerenciaHome) -> Command<MsgPrincipal> {
    match message {
        MsgGerenciaHome::MudarParaBusca => {
            *screen = Tela::Gerencia(TelaGerencia::Busca(DataGerencia::new()))
        }
        MsgGerenciaHome::MudarParaHome => *screen = Tela::Home,
    }

    Command::none()
}
