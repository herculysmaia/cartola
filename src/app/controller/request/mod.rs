use dotenv::dotenv;
use reqwest::Error;
use std::env;

use iced::Command;

use crate::app::model::{
    gerencia::{busca::MsgBuscaTime, MsgGerencia},
    DataTime, MsgPrincipal,
};

impl DataTime {
    pub fn buscar_time(termo: String) -> Command<MsgPrincipal> {
        Command::perform(chamar_endpoint(termo), |resposta| {
            MsgPrincipal::MsgGerencia(MsgGerencia::MsgBuscaTime(MsgBuscaTime::AtualizarResposta(
                resposta,
            )))
        })
    }

    pub async fn obter_imagem(url: String) -> Result<Option<Vec<u8>>, Error> {
        let imagem = match reqwest::get(url).await {
            Ok(result) => result,
            Err(e) => return Err(e),
        };
        match imagem.bytes().await {
            Ok(result) => return Ok(Some(result.to_vec())),
            Err(e) => return Err(e),
        };
    }
}

async fn chamar_endpoint(q: String) -> Vec<DataTime> {
    let mut retorno = Vec::new();

    dotenv().ok();

    let url = env::var("BUSCAR_TIME").unwrap_or_else(|e| {
        println!("Erro na obtenção da variável de ambiente: {e}");
        String::new()
    });

    if url.is_empty() {
        return retorno.to_owned();
    }

    let query = format!("{url}{q}");
    let criando_cliente = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/122.0.0.0 Safari/537.36 Edg/122.0.0.0")
        .build();

    let cliente = {
        match criando_cliente {
            Ok(cliente) => cliente,
            Err(e) => {
                println!("Erro ao criar cliente: {e:?}");
                return retorno;
            }
        }
    };

    let resposta = cliente.get(query).send().await.expect("Erro na requisição");

    if resposta.status().is_success() {
        let corpo = resposta.text().await.expect("Erro na obtenção do conteúdo");

        retorno = serde_json::from_str(&corpo).unwrap();

        return retorno.to_owned();
    } else {
        return retorno.to_owned();
    };
}
