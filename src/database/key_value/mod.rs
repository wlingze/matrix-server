mod user;

use crate::utility::error::Result;

pub type TupleOfByte = (Vec<u8>, Vec<u8>);

/// # key-value
///
pub trait KV: Send + Sync {
    // get value by key
    fn get(&self, key: &[u8]) -> Result<Option<Vec<u8>>>;

    fn insert(&self, key: &[u8], value: &[u8]) -> Result<()>;

    fn remove(&self, key: &[u8]) -> Result<()>;

    fn iter<'a>(&'a self) -> Box<dyn Iterator<Item = TupleOfByte> + 'a>;

    fn iter_form<'a>(
        &'a self,
        key_prefix: &str,
        from: &[u8],
    ) -> Box<dyn Iterator<Item = TupleOfByte> + 'a>;
}
