// api mod
// this part handle the api request to response.

pub mod send;

pub mod login;
pub mod recv;
pub mod register;

/// GET "/ping"
pub async fn ping(body: String) -> &'static str {
    "pong"
}
