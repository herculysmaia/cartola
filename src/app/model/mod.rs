// Publicizando dados das sub-páginas
pub mod gerencia;
pub mod home;

// Importando dados das sub-páginas
use gerencia::{MsgGerencia, TelaGerencia};
use home::MsgHome;

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
