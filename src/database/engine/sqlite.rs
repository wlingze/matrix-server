use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

use parking_lot::{Mutex, MutexGuard};
use rusqlite::{Connection, DatabaseName::Main, OptionalExtension};
use thread_local::ThreadLocal;

use crate::{
    config::Config,
    database::{
        engine::DBEngine,
        key_value::{TupleOfByte, KV},
    },
    utility::error::Result,
};

const DATABASE_FILE_NAME: &str = "conduit.db";

pub struct Engine {
    connect: Mutex<Connection>,
    read_connect: ThreadLocal<Connection>,
    // read_connect: Mutex<Connection>,
    path: PathBuf,
}

impl Engine {
    fn pre_open(path: &Path) -> Result<Connection> {
        let con = Connection::open(path)?;
        // con.pragma_update(Some(Main), "page_size", 2048)?;
        con.pragma_update(Some(Main), "synchronous", "NORMAL")?;
        con.pragma_update(Some(Main), "cache_size", 0)?;
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

        // // set thread local store
        let read_connect = ThreadLocal::new();
        read_connect.get_or(|| Engine::pre_open(&path).unwrap());

        Ok(Arc::new(Engine {
            connect,
            // read_connect: Mutex::new(Engine::pre_open(&path)?),
            read_connect,
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

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = TupleOfByte> + 'a> {
        // todo!()
        let guard = self.engine.read_lock();
        let statement = Box::leak(Box::new(
            guard
                .prepare(format!("SELECT key, value FROM {} ORDER BY key ASC", self.name).as_str())
                .unwrap(),
        ));
        let row = Box::new(
            statement
                .query_map([], |row| Ok((row.get_unwrap(0), row.get_unwrap(1))))
                .unwrap()
                .map(|f| f.unwrap()),
        );
        row
    }

    fn iter_form<'a>(
        &'a self,
        key_prefix: &str,
        from: &[u8],
    ) -> Box<dyn Iterator<Item = TupleOfByte> + 'a> {
        let guard = self.engine.read_lock();
        let statement = Box::leak(Box::new(
            guard
                .prepare(
                    format!(
                        "SELECT key, value FROM {} WHERE key LIKE '{}%' AND substr(key, {}) >= ? ORDER BY key ASC",
                        self.name,
                        key_prefix,
                        key_prefix.len() + 1
                    )
                    .as_str(),
                )
                .unwrap(),
        ));
        let row = Box::new(
            statement
                .query_map([from], |row| Ok((row.get_unwrap(0), row.get_unwrap(1))))
                .unwrap()
                .map(|f| f.unwrap()),
        );
        row
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
        // create_dir(tmp_dir).unwrap();
        create_dir(tmp_dir.clone()).map_err(|_| {
            remove_dir_all(tmp_dir.clone()).unwrap();
            create_dir(tmp_dir.clone()).unwrap()
        });
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

        {
            table.insert("user1".as_bytes(), &[1]).unwrap();
            table.insert("user2".as_bytes(), &[2]).unwrap();
            table.insert("user3".as_bytes(), &[3]).unwrap();
            table.insert("user4".as_bytes(), &[4]).unwrap();
            let mut iter = table.iter();

            // iter
            assert_eq!(
                iter.next(),
                Some(("user1".as_bytes().to_vec(), [1].to_vec()))
            );
            assert_eq!(
                iter.next(),
                Some(("user2".as_bytes().to_vec(), [2].to_vec()))
            );
            assert_eq!(
                iter.next(),
                Some(("user3".as_bytes().to_vec(), [3].to_vec()))
            );
            assert_eq!(
                iter.next(),
                Some(("user4".as_bytes().to_vec(), [4].to_vec()))
            );

            // iter from
            let mut iter_from = table.iter_form("user", "3".as_bytes());
            assert_eq!(
                iter_from.next(),
                Some(("user3".as_bytes().to_vec(), [3].to_vec()))
            );
            assert_eq!(
                iter_from.next(),
                Some(("user4".as_bytes().to_vec(), [4].to_vec()))
            );

            // iter from out-bound
            let mut iter_from_out_bound = table.iter_form("user", "5".as_bytes());
            assert_eq!(iter_from_out_bound.next(), None);
        }

        // delete /tmp/1/conduit.db
        remove_dir_all(tmp_dir).unwrap();
    }
}
