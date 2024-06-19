// Importando necessidades do Iced
use iced::{
    widget::{Button, Column, Text},
    Element,
};

// Importando necessidades locais
use crate::app::model::{
    gerencia::{home::MsgGerenciaHome, MsgGerencia},
    MsgPrincipal,
};

// Definindo tela de exibição
pub fn view() -> Element<'static, MsgPrincipal> {
    let texto_inicial = Text::new("Tela de Gerencia");

    // Botão para a tela de busca
    let btn_busca = Button::new("Buscar").on_press(MsgPrincipal::MsgGerencia(
        MsgGerencia::MsgGerenciaHome(MsgGerenciaHome::MudarParaBusca),
    ));

    // Botão para a tela anterior
    let btn_voltar = Button::new("Voltar").on_press(MsgPrincipal::MsgGerencia(
        MsgGerencia::MsgGerenciaHome(MsgGerenciaHome::MudarParaHome),
    ));

    // Diagramação da tela
    Column::new()
        .push(texto_inicial)
        .push(btn_busca)
        .push(btn_voltar)
        .into()
}
