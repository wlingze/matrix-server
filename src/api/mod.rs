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

fn token_check<F>(token: String, func: F) -> Result<()>
where
    F: Fn(String) -> bool,
{
    func(get_user_from_token(token)?)
        .then_some(())
        .ok_or(Error::BadRequest("Wrong token."))
}

fn get_user_from_token(token: String) -> Result<String> {
    services()
        .handler
        .from_token(token)?
        .ok_or(Error::BadRequest("Wrong token."))
}
