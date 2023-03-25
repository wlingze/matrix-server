pub mod sqlite;

use crate::config::Config;

use crate::database::key_value::KV;
use crate::utility::error::Result;

pub trait DBEngine {
    // open database engine
    fn open(config: Config) -> Result<Box<Self>>
    where
        Self: Sized;

    // open key-value tree
    fn open_tree(&self, name: &str) -> Result<Box<dyn KV>>;

    // flush data
    fn flush(&self) -> Result<()>;
}
