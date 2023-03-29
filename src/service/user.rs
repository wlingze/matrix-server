use ruma::UserId;

use crate::utility::error::Result;

pub trait Handler: Send + Sync {
    // check user exists
    fn exists(&self, user_id: &UserId) -> Result<bool>;

    // the number of users in the server
    fn count(&self) -> Result<usize>;

    // get password has for the given user
    fn get_password(&self, user_id: &UserId) -> Result<String>;

    // set password for the given user
    fn set_password(&self, user_id: &UserId, password: &str) -> Result<()>;

    // get user displayname for the given user
    fn get_displayname(&self, user_id: &UserId) -> Result<String>;

    // set user displayname for the given user
    fn set_displayname(&self, user_id: &UserId, displayname: &str) -> Result<()>;
}
