// Importando necessidades locais
use crate::app::model::{
    gerencia::{
        busca::{DataGerencia, MsgBuscaTime},
        TelaGerencia,
    },
    Tela,
};

// Implementação do comportamento dos dados da tela de busca
impl DataGerencia {
    pub fn new() -> Self {
        Self {
            entrada_pesquisa: String::new(),
        }
    }

    pub fn update(&mut self, texto: String) {
        self.entrada_pesquisa = texto;
    }
}

// Implementação do comportamento da tela de busca
pub fn update(screen: &mut Tela, message: MsgBuscaTime) {
    match message {
        MsgBuscaTime::AtualizarEntrada(texto) => {
            if let Tela::Gerencia(TelaGerencia::Busca(dados)) = screen {
                dados.update(texto);
            }
        }
    }
}
