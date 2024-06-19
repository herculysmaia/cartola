// Publicizando dados das sub-páginas de gerencia
pub mod busca;
pub mod home;

// Importando dados das sub-páginas de gerencia
use busca::{DataGerencia, MsgBuscaTime};
use home::MsgGerenciaHome;

// Definindo sub-páginas de gerencia
#[derive(Debug, Clone)]
pub enum TelaGerencia {
    Home,
    Busca(DataGerencia),
}

// Definindo mensagens geradas pela tela de gerencia
#[derive(Debug, Clone)]
pub enum MsgGerencia {
    MsgBuscaTime(MsgBuscaTime),
    MsgGerenciaHome(MsgGerenciaHome),
}
