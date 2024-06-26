// Publicizando dados das sub-páginas
pub mod gerencia;
pub mod home;

// Importando dados das sub-páginas
use gerencia::{MsgGerencia, TelaGerencia};
use home::MsgHome;

use serde::{Deserialize, Serialize};

// Dados principais da aplicação
#[derive(Debug)]
pub struct AppData {
    pub screen: Tela,
}

// Telas acessíveis pela tela inicial da aplicação
#[derive(Debug, Clone)]
pub enum Tela {
    Home,
    Gerencia(TelaGerencia),
}

// Mensagem geradas pela aplicação
#[derive(Debug, Clone)]
pub enum MsgPrincipal {
    MsgHome(MsgHome),
    MsgGerencia(MsgGerencia),
}

// Dodos de retorno da API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataTime {
    pub nome_cartola: String,
    pub slug: String,
    url_escudo_png: String,
    pub url_escudo_svg: String,
    pub foto_perfil: String,
    pub nome: String,
    facebook_id: Option<u64>,
    pub time_id: u64,
    assinante: bool,
    lgpd_removido: bool,
    lgpd_quarentena: bool,
}
