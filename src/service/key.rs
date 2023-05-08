use crate::utility::error::Result;

pub trait Handler: Send + Sync {
    // get other user public key
    fn get_key(&self, username: &str) -> Result<Option<Vec<u8>>>;

    // set public key to a user
    fn set_key(&self, username: &str, public_key: Vec<u8>) -> Result<()>;
}
