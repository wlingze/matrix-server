// api mod
// this part handle the api request to response.

use crate::{
    service::services,
    utility::error::{Error, Result},
};

pub mod key;
pub mod message;
pub mod user;

/// GET "/ping"
pub async fn ping() -> &'static str {
    "pong"
}

fn token_check(token: String, user: String) -> Result<()> {
    (get_user_from_token(token)? == user)
        .then_some(())
        .ok_or(Error::BadRequest("Wrong token."))
}

fn get_user_from_token(token: String) -> Result<String> {
    services()
        .handler
        .from_token(token)?
        .ok_or(Error::BadRequest("Wrong token."))
}
