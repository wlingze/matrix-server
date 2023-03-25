use std::path::Path;

use rusqlite::Connection;

use crate::{
    config::Config,
    database::{engine::DBEngine, key_value::KV},
    utility::error::{Error, Result},
};

pub struct Engine {
    pub connect: Connection,
}

impl DBEngine for Engine {
    fn open(config: Config) -> Result<Box<Self>> {
        let path = Path::new(&config.database_path).join("conduit.db");
        if !path.exists() {
            return Err(Error::bad_config(
                "Found sqlite at database_path, but is not specified in config.",
            ));
        }
        let connect = Connection::open(path)?;
        Ok(Box::new(Engine { connect }))
    }

    fn open_tree(&self, name: &str) -> Result<Box<dyn KV>> {
        todo!()
    }

    fn flush(&self) -> Result<()> {
        todo!()
    }
}
