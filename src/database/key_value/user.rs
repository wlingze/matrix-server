use ruma::UserId;

use crate::utility;
use crate::utility::error::Error;
use crate::{service::user::Handler, utility::error::Result};

use crate::database::Database;

const DEFAULT_PASSWORD: &[u8] = "".as_bytes();
const TOKEN_LENGTH: usize = 32;

impl Handler for Database {
    fn exists(&self, user_id: &UserId) -> Result<bool> {
        Ok(self.user_password.get(user_id.as_bytes())?.is_some())
    }

    fn get_password(&self, user_id: &UserId) -> Result<Option<String>> {
        self.user_password
            .get(user_id.as_bytes())?
            .map_or(Ok(None), |bytes| {
                Ok(Some(utility::string_from_bytes(bytes).map_err(|_| {
                    Error::bad_database(
                        "Password hash in db table user-password is not valid string.",
                    )
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

    fn set_token(&self, user_id: &UserId) -> Result<String> {
        // get old token  from user-token
        if let Some(token) = self.user_token.get(user_id.as_bytes())?.map_or(
            Ok::<Option<String>, Error>(None),
            |bytes| {
                Ok(Some(utility::string_from_bytes(bytes).map_err(|_| {
                    Error::bad_database("User ID in token-user is invalid unicode.")
                })?))
            },
        )? {
            // if old token exist remove
            self.user_token.remove(user_id.as_bytes())?;
            self.token_user.remove(token.as_bytes())?;
        }

        // get new token from random
        let token = utility::random_string(TOKEN_LENGTH);
        // set new token to user-token and token-user
        self.user_token
            .insert(user_id.as_bytes(), token.as_bytes())?;
        self.token_user
            .insert(token.as_bytes(), user_id.as_bytes())?;
        Ok(token)
    }

    fn from_token(&self, token: String) -> Result<Option<UserId>> {
        self.token_user
            .get(token.as_bytes())?
            .map_or(Ok(None), |bytes| {
                Ok(Some(
                    UserId::try_from(utility::string_from_bytes(bytes).map_err(|_| {
                        Error::bad_database("User ID in token-user is invalid unicode.")
                    })?)
                    .map_err(|_| Error::bad_database("User ID in token-user is invalid userid."))?,
                ))
            })
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

        // // get displayname -> None
        // assert_eq!(services().handler.get_displayname(&user).unwrap(), None);

        // // set displayname
        // services()
        //     .handler
        //     .set_displayname(&user, Some("carl"))
        //     .unwrap();
        // // get displayname -> Some
        // assert_eq!(
        //     services().handler.get_displayname(&user).unwrap(),
        //     Some("carl".to_string())
        // );

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

        // set token
        let token = services().handler.set_token(&user).unwrap();
        // get user from token
        assert_eq!(
            services().handler.from_token(token.clone()).unwrap(),
            Some(user.clone())
        );
        // set new token
        let new_token = services().handler.set_token(&user).unwrap();
        assert_eq!(
            services().handler.from_token(new_token).unwrap(),
            Some(user.clone())
        );
        // remove old token
        assert_eq!(services().handler.from_token(token.clone()).unwrap(), None);

        // check user -> true
        assert_eq!(services().handler.exists(&user).unwrap(), true);

        // delete
        remove_dir_all(tmp_dir).unwrap();
    }
}
