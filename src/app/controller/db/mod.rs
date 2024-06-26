// Importando necessidades do Rust
use rusqlite::{params, Connection};
use tokio::runtime::Runtime;

// Importando necessidades do Iced
use iced::Command;

// Importando necessidades locais
use crate::app::model::{gerencia::busca::DataGerencia, DataTime, MsgPrincipal};

// Implementando função para relacionadas a banco de dados na estrutura DateTime
impl DataTime {
    pub fn salvar_no_banco(&self, dados: &mut DataGerencia) -> Command<MsgPrincipal> {
        let conn = open_connection(dados);

        match tabela_existe(&conn) {
            Err(e) => println!("Erro: {e}"),
            _ => (),
        };

        let stmt = conn.prepare(
            "INSERT OR IGNORE INTO times (id, nome, cartola, escudo, perfil, slug, desde, indicado, ativo)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)"
        );

        match stmt {
            Ok(mut result) => {
                let id = self.time_id;
                let nome = self.nome.clone();
                let cartola = self.nome_cartola.clone();
                let slug = self.slug.clone();
                let mut escudo = Some(Vec::new());
                let mut perfil = Some(Vec::new());

                match Runtime::new() {
                    Ok(run) => {
                        escudo = match run.block_on(async {
                            DataTime::obter_imagem(self.url_escudo_svg.clone()).await
                        }) {
                            Ok(result) => result,
                            Err(e) => {
                                println!("Erro: {e}");
                                Some(Vec::new())
                            }
                        };
                        perfil = match run.block_on(async {
                            DataTime::obter_imagem(self.foto_perfil.clone()).await
                        }) {
                            Ok(result) => result,
                            Err(e) => {
                                println!("Erro: {e}");
                                Some(Vec::new())
                            }
                        };
                    }
                    Err(e) => println!("Erro: {e}"),
                };

                match result.execute(params![id, nome, cartola, escudo, perfil, slug, 2024, 0, 1]) {
                    Err(e) => println!("Erro ao inserir: {e}"),
                    _ => (),
                }
            }
            Err(e) => println!("Erro na preparação da inserção: {e}"),
        };
        Command::none()
    }
}

// Abrir conexão com o banco de dados
fn open_connection(dados: &mut DataGerencia) -> Connection {
    Connection::open(dados.dir_db.clone()).unwrap()
}

// Verificando se a tabela já existe e cria caso contrário
fn tabela_existe(conn: &Connection) -> Result<(), rusqlite::Error> {
    let mut stmt =
        match conn.prepare("SELECT name FROM sqlite_master WHERE type='table' AND name='times';") {
            Ok(result) => result,
            Err(e) => return Err(e),
        };

    let existe = match stmt.exists([]) {
        Ok(result) => result,
        Err(e) => return Err(e),
    };

    if !existe {
        match conn.execute(
            "CREATE TABLE times (
                id	INTEGER NOT NULL UNIQUE,
                nome	TEXT NOT NULL,
                cartola	TEXT NOT NULL,
                escudo	BLOB NOT NULL,
                perfil	BLOB,
                slug	TEXT NOT NULL,
                desde	INTEGER,
                indicado	INTEGER,
                ativo	INTEGER NOT NULL,
                PRIMARY KEY(id)
            );",
            [],
        ) {
            Ok(_) => return Ok(()),
            Err(e) => return Err(e),
        };
    };

    Ok(())
}
