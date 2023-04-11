use crate::utility::error::Error;
use crate::utility::{self, password_encode};
use crate::{service::user::Handler, utility::error::Result};

use crate::database::Database;

const DEFAULT_PASSWORD: &str = "";
const TOKEN_LENGTH: usize = 32;

impl Handler for Database {
    fn exists(&self, user_id: &String) -> Result<bool> {
        Ok(self.user_password.get(user_id.as_bytes())?.is_some())
    }

    fn get_password(&self, user_id: &String) -> Result<Option<String>> {
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

    fn set_password(&self, user_id: &String, password: Option<&str>) -> Result<()> {
        let password = match password {
            Some(password) => password_encode(password)
                .map_err(|_| Error::BadRequest("password is invalid unicode"))?,
            None => DEFAULT_PASSWORD.to_string(),
        };
        self.user_password
            .insert(user_id.as_bytes(), password.as_bytes())?;
        Ok(())
    }

    fn set_token(&self, user_id: &String) -> Result<String> {
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

    fn from_token(&self, token: String) -> Result<Option<String>> {
        self.token_user
            .get(token.as_bytes())?
            .map_or(Ok(None), |bytes| {
                Ok(Some(
                    String::try_from(utility::string_from_bytes(bytes).map_err(|_| {
                        Error::bad_database("User ID in token-user is invalid unicode.")
                    })?)
                    .map_err(|_| Error::bad_database("User ID in token-user is invalid userid."))?,
                ))
            })
    }

    fn users(&self) -> Result<Vec<String>> {
        Ok(self
            .user_password
            .iter()
            .map(|tuple| String::from_utf8(tuple.0.to_vec()).unwrap())
            .collect())
    }
}
