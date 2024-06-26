// Importando necessidades locais
use crate::app::model::DataTime;

// Dados necessários a tela de busca
#[derive(Debug, Clone)]
pub struct DataGerencia {
    pub entrada_pesquisa: String,
    pub lista_times: Vec<DataTime>,
    pub dir_db: String,
}

// Mensagens geradas pela tela de busca
#[derive(Debug, Clone)]
pub enum MsgBuscaTime {
    AtualizarEntrada(String),
    AtualizarResposta(Vec<DataTime>),
    AddSubData(DataTime),
}
