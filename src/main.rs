mod app {
    pub mod gerenciamento {
        pub mod cadastro;
    }
}
use app::gerenciamento::cadastro::TelaDeCadastro;

use iced::widget::Container;
use iced::Length;
use iced::{executor, Application, Command, Element, Settings, Theme};

struct CartolaApp {
    tela: Telas,
    query: TelaDeCadastro,
}

#[derive(Debug, Clone, Copy)]
enum Telas {
    Gerenciamento(TelaDeGerencia),
}

#[derive(Debug, Clone, Copy)]
enum TelaDeGerencia {
    Cadastro,
}

#[derive(Debug, Clone)]
enum Message {
    TextInputChanged(String),
}

impl Application for CartolaApp {
    type Executor = executor::Default;
    type Theme = Theme;
    type Flags = ();
    type Message = Message;

    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (
            Self {
                query: TelaDeCadastro::new(),
                tela: Telas::Gerenciamento(TelaDeGerencia::Cadastro),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        String::from("Cartola")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::TextInputChanged(input) => {
                self.query.update_query(input);
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let content = match self.tela {
            Telas::Gerenciamento(TelaDeGerencia::Cadastro) => self.query.view(),
        };

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

fn main() -> iced::Result {
    CartolaApp::run(Settings::default())
}
