pub mod sqlite;

use std::sync::Arc;

use crate::config::Config;

use crate::database::key_value::KV;
use crate::utility::error::Result;

pub trait DBEngine {
    // open database engine
    fn open(config: Config) -> Result<Self>
    where
        Self: Sized;

    // open key-value tree
    fn open_tree(&self, name: &str) -> Result<Arc<dyn KV>>;

    // flush data
    fn flush(&self) -> Result<()>;
}
