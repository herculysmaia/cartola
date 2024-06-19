// Importando necessidades do Iced
use iced::{
    widget::{Column, TextInput},
    Element,
};

// Importando necessidades locais
use crate::app::model::{
    gerencia::{
        busca::{DataGerencia, MsgBuscaTime},
        MsgGerencia,
    },
    MsgPrincipal,
};

// Definido tela de exibição
pub fn view(dados: &DataGerencia) -> Element<'static, MsgPrincipal> {
    let campo_de_pesquisa = TextInput::new("Entre com o nome do time...", &dados.entrada_pesquisa)
        .on_input(|entrada| {
            MsgPrincipal::MsgGerencia(MsgGerencia::MsgBuscaTime(MsgBuscaTime::AtualizarEntrada(
                entrada,
            )))
        });

    // Diagramação da tela
    Column::new().push(campo_de_pesquisa).into()
}
