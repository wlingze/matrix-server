use crate::{service::user::Handler, utility::error::Result};

use crate::database::Database;

impl Handler for Database {
    fn exists(&self, user_id: &ruma::UserId) -> Result<bool> {
        println!("user exists: {}", user_id);
        todo!()
    }

    fn count(&self) -> Result<usize> {
        println!("user count: {}", 0);
        todo!()
    }

    fn get_password(&self, user_id: &ruma::UserId) -> Result<String> {
        println!("user get password: {}", user_id);
        todo!()
    }

    fn set_password(&self, user_id: &ruma::UserId, password: &str) -> Result<()> {
        println!("user set password: {} {}", user_id, password);
        todo!()
    }

    fn get_displayname(&self, user_id: &ruma::UserId) -> Result<String> {
        println!("user get displayname: {}", user_id);
        todo!()
    }

    fn set_displayname(&self, user_id: &ruma::UserId, displayname: &str) -> Result<()> {
        println!("user set displayname: {} {}", user_id, displayname);
        todo!()
    }
}
