use ruma::UserId;

use crate::utility::error::Result;

pub trait Handler: Send + Sync {
    // check user exists
    fn exists(&self, user_id: &UserId) -> Result<bool>;

    // password
    // get password has for the given user
    fn get_password(&self, user_id: &UserId) -> Result<Option<String>>;
    // set password for the given user
    fn set_password(&self, user_id: &UserId, password: Option<&str>) -> Result<()>;

    // displayname
    // get user displayname for the given user
    fn get_displayname(&self, user_id: &UserId) -> Result<Option<String>>;
    // set user displayname for the given user
    fn set_displayname(&self, user_id: &UserId, displayname: Option<&str>) -> Result<()>;
}
