// Importando necessidades locais
use crate::app::model::{
    gerencia::{busca::DataGerencia, home::MsgGerenciaHome, TelaGerencia},
    Tela,
};

// Definindo comportamento da tela inicial de gerencia
pub fn update(message: MsgGerenciaHome, screen: &mut Tela) {
    match message {
        MsgGerenciaHome::MudarParaBusca => {
            *screen = Tela::Gerencia(TelaGerencia::Busca(DataGerencia::new()))
        }
        MsgGerenciaHome::MudarParaHome => *screen = Tela::Home,
    }
}
