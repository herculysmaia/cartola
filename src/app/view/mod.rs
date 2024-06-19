// Importando telas acessíveis no estado atual da janela
mod gerencia;
mod home;

// Importando componentes do Iced
use iced::Element;

// Importando necessidades locais
use crate::app::model::{MsgPrincipal, Tela};

// Implementação do comportamento das telas
impl Tela {
    pub fn view(&self) -> Element<MsgPrincipal> {
        match self {
            Tela::Home => home::view(),
            Tela::Gerencia(tela) => gerencia::view(tela),
        }
    }
}
