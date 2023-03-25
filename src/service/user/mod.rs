use ruma::UserId;

use crate::utility::error::Result;

pub struct Services {
    pub handler: &'static dyn Handler,
}

impl Services {
    pub fn exists(&self, user_id: &UserId) -> Result<bool> {
        self.handler.exists(user_id)
    }

    pub fn count(&self) -> Result<usize> {
        self.handler.count()
    }

    // password
    pub fn get_password(&self, user_id: &UserId) -> Result<String> {
        self.handler.get_password(user_id)
    }

    pub fn set_password(&self, user_id: &UserId, password: &str) -> Result<()> {
        self.handler.set_password(user_id, password)
    }

    // display name
    pub fn get_displayname(&self, user_id: &UserId) -> Result<String> {
        self.handler.get_displayname(user_id)
    }

    pub fn set_displayname(&self, user_id: &UserId, displayname: &str) -> Result<()> {
        self.handler.set_displayname(user_id, displayname)
    }
}

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
