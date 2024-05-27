use iced::widget::{Column, Container, TextInput};
use iced::Length;
use iced::{executor, Application, Command, Element, Settings, Theme};

struct CartolaApp {
    query: String,
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
        let app = Self {
            query: String::new(),
        };
        (app, Command::none())
    }

    fn title(&self) -> String {
        String::from("Cartola")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::TextInputChanged(input) => {
                self.query = input;
            }
        }
        Command::none()
    }

    fn view(&self) -> Element<Self::Message> {
        let input = TextInput::new("hover", &self.query)
            .on_input(Message::TextInputChanged)
            .padding(10);

        let content = Column::new().push(input).spacing(20);

        Container::new(content)
            .width(Length::Fill)
            .center_x()
            .into()
    }
}

fn main() -> iced::Result {
    CartolaApp::run(Settings::default())
}
