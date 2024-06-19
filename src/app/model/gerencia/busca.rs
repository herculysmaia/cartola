// Dados necessários a tela de busca
#[derive(Debug, Clone)]
pub struct DataGerencia {
    pub entrada_pesquisa: String,
}

// Mensagens geradas pela tela de busca
#[derive(Debug, Clone)]
pub enum MsgBuscaTime {
    AtualizarEntrada(String),
}
