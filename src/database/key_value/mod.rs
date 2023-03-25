mod user;

use crate::utility::error::Result;

/// # key-value
///
pub trait KV: Send + Sync {
    // get value by key
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>>;

    fn insert(&self, key: &[u8], value: &[u8]) -> Result<()>;

    fn remove(&self, key: &[u8]) -> Result<()>;
}
