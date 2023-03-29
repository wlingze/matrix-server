use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use parking_lot::{Mutex, MutexGuard};
use rusqlite::{Connection, DatabaseName::Main, OptionalExtension};
use thread_local::ThreadLocal;

use crate::{
    config::Config,
    database::{engine::DBEngine, key_value::KV},
    utility::error::Result,
};

const DATABASE_FILE_NAME: &str = "conduit.db";

pub struct Engine {
    connect: Mutex<Connection>,
    read_connect: ThreadLocal<Connection>,
    path: PathBuf,
}

impl Engine {
    fn pre_open(path: &Path) -> Result<Connection> {
        let con = Connection::open(path)?;
        con.pragma_update(Some(Main), "page_size", 2048)?;
        // con.pragma_update(Some(Main), "journal_mode", "WAL")?;
        con.pragma_update(Some(Main), "synchronous", "NORMAL")?;
        Ok(con)
    }

    fn write_lock(&self) -> MutexGuard<'_, Connection> {
        self.connect.lock()
    }

    fn read_lock(&self) -> &Connection {
        self.read_connect
            .get_or(|| Engine::pre_open(&self.path).unwrap())
    }
}

impl DBEngine for Arc<Engine> {
    fn open(config: Config) -> Result<Self> {
        let path = Path::new(&config.database_path).join(DATABASE_FILE_NAME);
        let connect = Mutex::new(Engine::pre_open(&path)?);
        Ok(Arc::new(Engine {
            connect,
            read_connect: ThreadLocal::new(),
            path,
        }))
    }

    fn open_tree(&self, name: &str) -> Result<Arc<dyn KV>> {
        self.write_lock().execute(
            format!(
                "CREATE TABLE IF NOT EXISTS {} (
                \"key\" BLOB PRIMARY KEY,
                \"value\" BLOB NOT NULL
            )",
                name
            )
            .as_str(),
            [],
        )?;
        Ok(Arc::new(Table {
            engine: Arc::clone(self),
            name: name.to_string(),
        }))
    }

    fn flush(&self) -> Result<()> {
        // flush the database with rusqlite
        self.write_lock().execute("VACUUM", [])?;
        Ok(())
    }
}

pub struct Table {
    engine: Arc<Engine>,
    name: String,
}

impl KV for Table {
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>> {
        Ok(self
            .engine
            .read_lock()
            .prepare(format!("SELECT value FROM {} WHERE key = ?", self.name).as_str())?
            .query_row([key], |row| row.get(0))
            .optional()?)
    }

    fn insert(&self, key: &[u8], value: &[u8]) -> Result<()> {
        self.engine.write_lock().execute(
            format!(
                "INSERT OR REPLACE INTO {} (key, value) VALUES (?, ?)",
                self.name
            )
            .as_str(),
            [key, value],
        )?;
        Ok(())
    }

    fn remove(&self, key: &[u8]) -> Result<()> {
        self.engine.write_lock().execute(
            format!("DELETE FROM {} WHERE key = ?", self.name).as_str(),
            [key],
        )?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::fs::{create_dir, remove_dir_all};

    use crate::config;

    use super::*;
    #[test]
    fn test_sqlite() {
        // Engine test

        // Engine open
        let mut conf = config::default();
        // set tmp dir
        let tmp_dir = "/tmp/test_sqlite";
        create_dir(tmp_dir).unwrap();
        conf.database_path = tmp_dir.to_string();

        let engine = Arc::<Engine>::open(conf).unwrap();
        // Engine open_tree
        let table = engine.open_tree("test_table").unwrap();

        // Table test

        let key = "test_key".as_bytes();
        // Table get = None
        {
            assert_eq!(table.get(key).unwrap(), None);
        }

        // Table insert
        {
            table.insert(key, &[1]).unwrap();
        }
        // Table get = 1
        {
            let row = table.get(key).unwrap().unwrap();
            assert_eq!(row.len(), 1);
            assert_eq!(row[0], 1);
        }
        // Table remove
        {
            table.remove(key).unwrap();
            // Table get = 0
            assert_eq!(table.get(key).unwrap(), None);
        }

        // delete /tmp/1/conduit.db
        remove_dir_all(tmp_dir).unwrap();
    }
}
