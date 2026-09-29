use redb::{Database, ReadableTable, TableDefinition};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::path::Path;
use std::sync::Arc;

// Tabela única para credenciais: chave (&str) -> payload de bytes (&[u8])
const CREDENTIALS_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("credentials");

pub struct AppDatabase {
    db: Arc<Database>,
}

impl AppDatabase {
    /// Abre ou cria o arquivo do banco no caminho especificado
    pub fn init<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let db = Database::create(path)?;
        
        // Garante que a tabela exista abrindo uma transação rápida de escrita
        let write_txn = db.begin_write()?;
        {
            let _ = write_txn.open_table(CREDENTIALS_TABLE)?;
        }
        write_txn.commit()?;

        Ok(Self { db: Arc::new(db) })
    }

    /// Salva qualquer estrutura serializável sob uma chave específica
    pub fn save_item<T: Serialize>(&self, key: &str, item: &T) -> Result<(), Box<dyn std::error::Error>> {
        let bytes = serde_json::to_vec(item)?;

        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(CREDENTIALS_TABLE)?;
            table.insert(key, bytes.as_slice())?;
        }
        write_txn.commit()?;
        Ok(())
    }

    /// Recupera e desserializa uma estrutura pela chave
    pub fn get_item<T: DeserializeOwned>(&self, key: &str) -> Result<Option<T>, Box<dyn std::error::Error>> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(CREDENTIALS_TABLE)?;

        if let Some(access) = table.get(key)? {
            let item: T = serde_json::from_slice(access.value())?;
            return Ok(Some(item));
        }

        Ok(None)
    }
}