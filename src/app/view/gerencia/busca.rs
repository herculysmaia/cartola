// Importando necessidades do Iced
use iced::{
    widget::{Button, Column, Row, Text, TextInput},
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

    let mut tela = Column::new().push(campo_de_pesquisa);

    for time in &dados.lista_times {
        let nome = &time.nome;
        let wid_nome = Text::new(nome.clone());

        let btn_add_sub = Button::new("+").on_press(MsgPrincipal::MsgGerencia(
            MsgGerencia::MsgBuscaTime(MsgBuscaTime::AddSubData(time.clone())),
        ));

        let resultado = Row::new().push(wid_nome).push(btn_add_sub);

        tela = tela.push(resultado);
    }

    // Diagramação da tela
    tela.into()
}
