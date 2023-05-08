use crate::{database::Database, service::key::Handler, utility::error::Result};
use base64;

impl Handler for Database {
    fn get_key(&self, username: &str) -> Result<Option<Vec<u8>>> {
        self.user_key
            .get(username.as_bytes())
            .map(|bytes| bytes.map(|bytes| base64::decode(bytes).unwrap()))
    }

    fn set_key(&self, username: &str, public_key: Vec<u8>) -> Result<()> {
        let public_key_str = base64::encode(&public_key);
        self.user_key
            .insert(username.as_bytes(), &public_key_str.as_bytes())?;
        Ok(())
    }
}
