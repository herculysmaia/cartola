// Importando necessidades do Iced
use iced::{
    widget::{Button, Column, Text},
    Element,
};

// Importando necessidades locais
use crate::app::model::{home::MsgHome, MsgPrincipal};

// Definindo a tela de exibição
pub fn view() -> Element<'static, MsgPrincipal> {
    let texto_inicial = Text::new("Tela inicial");

    // Botão que leva a tela de gerencia
    let btn_gerencia = Button::new(Text::new("Gerencia"))
        .on_press(MsgPrincipal::MsgHome(MsgHome::MudarParaGerencia));

    // Diagramação da tela
    Column::new().push(texto_inicial).push(btn_gerencia).into()
}
