// api mod
// this part handle the api request to response.

pub mod login;
pub mod recv;
pub mod register;
pub mod send;

/// GET "/ping"
pub async fn ping(body: String) -> &'static str {
    "pong"
}
