// Importando necessidades do Rust
use std::path::Path;
use std::{env, fs};

// Importando necessidades do Iced
use iced::Command;

// Importando necessidades locais
use crate::app::model::{
    gerencia::{
        busca::{DataGerencia, MsgBuscaTime},
        TelaGerencia,
    },
    DataTime, MsgPrincipal, Tela,
};

// Implementação do comportamento dos dados da tela de busca
impl DataGerencia {
    pub fn new() -> Self {
        // REFATORAR // REFATORAR // REFATORAR //
        // Obtenção do diretório de execução
        let dir_path = match env::current_dir() {
            Ok(caminho) => caminho,
            Err(e) => {
                println!("Erro ao obter diretório atual: {e}");
                std::process::exit(1);
            }
        };

        // Transformando PathBuf do caminho atual em String
        let dir = dir_path
            .to_str()
            .map(|s| s.to_string())
            .unwrap_or(String::from("C:\\"));

        let dir_db = format!("{dir}\\db");

        // Verificando se o caminho já foi existe e cria caso contrário
        if !Path::new(dir_db.as_str()).exists() {
            match fs::create_dir(dir_db.clone()) {
                Err(e) => println!("Não foi possível criar o diretório: {e}"),
                _ => (),
            }
        }

        let dir_db = format!("{dir_db}\\2024.db");
        // REFATORAR // REFATORAR // REFATORAR //

        Self {
            entrada_pesquisa: String::new(),
            lista_times: Vec::new(),
            dir_db,
        }
    }

    // Função para atualizar o texto do campo de pesquisa
    pub fn atualizar_campo_de_texto(&mut self, texto: String) -> Command<MsgPrincipal> {
        self.entrada_pesquisa = texto.clone();
        DataTime::buscar_time(texto)
    }

    // Função para atualizar a lista de resposta
    pub fn atualizar_campo_de_resposta(&mut self, entrada: Vec<DataTime>) -> Command<MsgPrincipal> {
        self.lista_times = entrada;
        Command::none()
    }
}

// Implementação do comportamento da tela de busca
pub fn update(screen: &mut Tela, message: MsgBuscaTime) -> Command<MsgPrincipal> {
    match message {
        MsgBuscaTime::AtualizarEntrada(texto) => {
            if let Tela::Gerencia(TelaGerencia::Busca(dados)) = screen {
                dados.atualizar_campo_de_texto(texto)
            } else {
                Command::none()
            }
        }
        MsgBuscaTime::AtualizarResposta(entrada) => {
            if let Tela::Gerencia(TelaGerencia::Busca(dados)) = screen {
                dados.atualizar_campo_de_resposta(entrada)
            } else {
                Command::none()
            }
        }
        MsgBuscaTime::AddSubData(entrada) => {
            if let Tela::Gerencia(TelaGerencia::Busca(dados)) = screen {
                entrada.salvar_no_banco(dados)
            } else {
                Command::none()
            }
        }
    }
}
