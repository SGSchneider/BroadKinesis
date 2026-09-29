use redb::{Database, ReadableDatabase, TableDefinition};
use serde::de::DeserializeOwned;
use serde::Serialize;
use std::path::Path;
use std::sync::Arc;

// Single credentials table: key (&str) -> byte payload (&[u8])
const CREDENTIALS_TABLE: TableDefinition<&str, &[u8]> = TableDefinition::new("credentials");

#[derive(Clone)]
pub struct AppDatabase {
    db: Arc<Database>,
}

impl AppDatabase {
    /// Opens or creates the database file at the specified path.
    pub fn init<P: AsRef<Path>>(path: P) -> Result<Self, Box<dyn std::error::Error>> {
        let db = Database::create(path)?;

        // Ensure the table exists by opening a short write transaction.
        let write_txn = db.begin_write()?;
        {
            let _ = write_txn.open_table(CREDENTIALS_TABLE)?;
        }
        write_txn.commit()?;

        Ok(Self { db: Arc::new(db) })
    }

    /// Saves any serializable structure under a specific key.
    pub fn save_item<T: Serialize>(
        &self,
        key: &str,
        item: &T,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let bytes = serde_json::to_vec(item)?;

        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(CREDENTIALS_TABLE)?;
            table.insert(key, bytes.as_slice())?;
        }
        write_txn.commit()?;
        Ok(())
    }

    /// Retrieves and deserializes a structure by key.
    pub fn get_item<T: DeserializeOwned>(
        &self,
        key: &str,
    ) -> Result<Option<T>, Box<dyn std::error::Error>> {
        let read_txn = self.db.begin_read()?;
        let table = read_txn.open_table(CREDENTIALS_TABLE)?;

        if let Some(access) = table.get(key)? {
            let item: T = serde_json::from_slice(access.value())?;
            return Ok(Some(item));
        }

        Ok(None)
    }

    pub fn delete_item(&self, key: &str) -> Result<(), Box<dyn std::error::Error>> {
        let write_txn = self.db.begin_write()?;
        {
            let mut table = write_txn.open_table(CREDENTIALS_TABLE)?;
            table.remove(key)?;
        }
        write_txn.commit()?;
        Ok(())
    }
}
