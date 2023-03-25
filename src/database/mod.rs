mod engine;
mod key_value;

use std::path::Path;

use crate::{
    config::Config,
    utility::error::{Error, Result},
};

use self::{engine::DBEngine, key_value::KV};

pub fn build_database(config: Config) -> Result<Box<Database>> {
    // builder
    let build = match config.database_backend.as_str() {
        "sqlite" => {
            #[cfg(not(feature = "sqlite"))]
            return Err(Error::bad_config("sqlite is not enabled"));

            #[cfg(feature = "sqlite")]
            engine::sqlite::Engine::open(config)?
        }
        &_ => return Err(Error::bad_config("Database backend not found.")),
    };

    // return
    Ok(Box::new(Database {
        user: build.open_tree("user")?,
    }))
}

pub struct Database {
    // todo
    // user
    pub user: Box<dyn KV>,
}
