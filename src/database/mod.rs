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
        token_user: build.open_tree("token_user")?,
        user_token: build.open_tree("user_token")?,
        user_count: build.open_tree("user_count")?,
        user_messageid: build.open_tree("user_messageid")?,
        messageid_message: build.open_tree("messageid_message")?,
        user_key: build.open_tree("user_key")?,
    }))
}

pub struct Database {
    // user
    pub user_password: Arc<dyn KV>,
    pub token_user: Arc<dyn KV>,
    pub user_token: Arc<dyn KV>,

    // message
    // user_count
    pub user_count: Arc<dyn KV>,
    // {username}{count} - messageid
    pub user_messageid: Arc<dyn KV>,
    // messageid - message
    pub messageid_message: Arc<dyn KV>,

    // public key
    pub user_key: Arc<dyn KV>,
}

impl Handler for Database {}
