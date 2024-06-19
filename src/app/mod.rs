// Aplicação do padrão model-view-controller
mod controller;
mod model;
mod view;

// Importar componentes do Iced
use iced::{executor, Application, Command, Settings, Theme};

// Importar necessidade locais
use model::{AppData, MsgPrincipal, Tela};

// Iniciar aplicação e tratar os erros
pub fn iniciar_app() {
    let settings = Settings::default();
    let response = AppData::run(settings);

    // Tratando a resposta após a execução da aplicação
    match response {
        Ok(dado) => dado,
        Err(erro) => println!("{erro:?}"),
    }
}

// Implementação principal do programa
impl Application for AppData {
    type Executor = executor::Default;
    type Flags = ();
    type Message = MsgPrincipal;
    type Theme = Theme;

    // Função inicializadora
    fn new(_flags: Self::Flags) -> (Self, iced::Command<Self::Message>) {
        (Self { screen: Tela::Home }, Command::none())
    }

    // Função para definir o titulo da aplicação
    fn title(&self) -> String {
        String::from("Cartola App")
    }

    // Função para atualizar componentes ativos
    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        self.screen.update(message)
    }

    // Função para exibir itens ativos
    fn view(&self) -> iced::Element<'_, Self::Message, Self::Theme, iced::Renderer> {
        self.screen.view()
    }
}
