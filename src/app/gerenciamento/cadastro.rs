use crate::Message;

use iced::widget::{Column, TextInput};
use iced::Element;

pub struct TelaDeCadastro {
    query: String,
}

impl TelaDeCadastro {
    pub fn new() -> Self {
        Self {
            query: String::new(),
        }
    }

    pub fn view(&self) -> Element<Message> {
        let input = TextInput::new("hover", &self.query)
            .on_input(Message::TextInputChanged)
            .padding(10);

        Column::new().push(input).spacing(10).into()
    }

    pub fn update_query(&mut self, input: String) {
        self.query = input;
    }
}
