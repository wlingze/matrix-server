// api mod
// this part handle the api request to response.

pub mod key;
pub mod message;
pub mod user;

/// GET "/ping"
pub async fn ping() -> &'static str {
    "pong"
}
