// Importando telas acessíveis no estado atual da janela
mod busca;
mod home;

// Importando necessidades do Iced
use iced::Element;

// Importando necessidades locais
use crate::app::model::{gerencia::TelaGerencia, MsgPrincipal};

// Buscando informações da tela que está atualmente em exibição
pub fn view(screen: &TelaGerencia) -> Element<'static, MsgPrincipal> {
    match screen {
        TelaGerencia::Home => home::view(),
        TelaGerencia::Busca(dados) => busca::view(dados),
    }
}
