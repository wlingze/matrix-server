mod engine;
mod key_value;

use std::sync::Arc;

use crate::{
    config::Config,
    service::services::Handler,
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
            Arc::<engine::sqlite::Engine>::open(config)?
        }
        &_ => return Err(Error::bad_config("Database backend not found.")),
    };

    // return
    Ok(Box::new(Database {
        user_password: build.open_tree("user_password")?,
        user_displayname: build.open_tree("user_displayname")?,
    }))
}

pub struct Database {
    // user
    pub user_password: Arc<dyn KV>,
    pub user_displayname: Arc<dyn KV>,
}

impl Handler for Database {}
