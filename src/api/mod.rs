// api mod
// this part handle the api request to response.

pub mod login;
pub mod recv;
pub mod register;
pub mod send;
pub mod user;

/// GET "/ping"
pub async fn ping() -> &'static str {
    "pong"
}
