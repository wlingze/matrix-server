// api mod
// this part handle the api request to response.

/// GET "/ping"
pub async fn ping() -> &'static str {
    "pong"
}
