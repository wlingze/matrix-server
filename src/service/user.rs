use crate::utility::error::Result;

pub trait Handler: Send + Sync {
    // check user exists
    fn exists(&self, user_id: &String) -> Result<bool>;

    // password
    // get password has for the given user
    fn get_password(&self, user_id: &String) -> Result<Option<String>>;
    // set password for the given user
    fn set_password(&self, user_id: &String, password: Option<&str>) -> Result<()>;

    // // displayname
    // // get user displayname for the given user
    // fn get_displayname(&self, user_id: &String) -> Result<Option<String>>;
    // // set user displayname for the given user
    // fn set_displayname(&self, user_id: &String, displayname: Option<&str>) -> Result<()>;

    // token
    // set token-user in login
    fn set_token(&self, user_id: &String) -> Result<String>;
    // get user from token
    fn from_token(&self, token: String) -> Result<Option<String>>;
}


