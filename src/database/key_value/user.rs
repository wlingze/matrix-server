use ruma::UserId;

use crate::utility::error::Error;
use crate::{service::user::Handler, utility::error::Result};

use crate::database::Database;

const DEFAULT_PASSWORD: &[u8] = "".as_bytes();

impl Handler for Database {
    fn exists(&self, user_id: &UserId) -> Result<bool> {
        Ok(self.user_password.get(user_id.as_bytes())?.is_some())
    }

    fn get_password(&self, user_id: &UserId) -> Result<Option<String>> {
        self.user_password
            .get(user_id.as_bytes())?
            .map_or(Ok(None), |bytes| {
                Ok(Some(String::from_utf8(bytes).map_err(|_| {
                    Error::bad_database("Password hash in db is not valid string.")
                })?))
            })
    }

    fn set_password(&self, user_id: &UserId, password: Option<&str>) -> Result<()> {
        let password = match password {
            Some(password) => password.as_bytes(),
            None => DEFAULT_PASSWORD,
        };
        self.user_password.insert(user_id.as_bytes(), password)?;
        Ok(())
    }

    fn get_displayname(&self, user_id: &UserId) -> Result<Option<String>> {
        self.user_displayname
            .get(user_id.as_bytes())?
            .map_or(Ok(None), |bytes| {
                Ok(Some(String::from_utf8(bytes).map_err(|_| {
                    Error::bad_database("Display name in db is not valid string.")
                })?))
            })
    }

    // set display-name, or remove it if displayname is None
    fn set_displayname(&self, user_id: &UserId, displayname: Option<&str>) -> Result<()> {
        if let Some(displayname) = displayname {
            self.user_displayname
                .insert(user_id.as_bytes(), displayname.as_bytes())?;
        } else {
            self.user_displayname.remove(user_id.as_bytes())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use std::fs::remove_dir_all;

    use crate::service::{services, test::setup_services};

    use super::*;
    #[test]
    fn test_user() {
        let tmp_dir = setup_services("test_user");
        let user = UserId::try_from("@carl:example.com").expect("Failed to create UserId.");

        // check user -> false
        assert_eq!(services().handler.exists(&user).unwrap(), false);

        // get password -> None
        assert_eq!(services().handler.get_password(&user).unwrap(), None);

        // get displayname -> None
        assert_eq!(services().handler.get_displayname(&user).unwrap(), None);

        // set displayname
        services()
            .handler
            .set_displayname(&user, Some("carl"))
            .unwrap();
        // get displayname -> Some
        assert_eq!(
            services().handler.get_displayname(&user).unwrap(),
            Some("carl".to_string())
        );

        // set password
        services()
            .handler
            .set_password(&user, Some("password"))
            .unwrap();
        // get displayname -> Some
        assert_eq!(
            services().handler.get_password(&user).unwrap(),
            Some("password".to_string())
        );

        // check user -> true
        assert_eq!(services().handler.exists(&user).unwrap(), true);
        // remove user
        services().handler.set_displayname(&user, None).unwrap();
        // get displayname -> None
        assert_eq!(services().handler.get_displayname(&user).unwrap(), None);

        // delete
        remove_dir_all(tmp_dir).unwrap();
    }
}
