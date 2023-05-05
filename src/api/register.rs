use serde::{Deserialize, Serialize};

use crate::{service::services, utility::error::Result};
use axum::extract::Json;

#[derive(Deserialize)]
pub struct Body {
    username: String,
    password: Option<String>,
}

#[derive(Serialize)]
pub struct Response {
    token: String,
}
/// # `POST /api/v0/register`
///
/// register an account on this server
///
pub async fn register_route(
    Json(Body { username, password }): Json<Body>,
) -> Result<Json<Response>> {
    // create user
    // set user-password
    services()
        .handler
        .set_password(&username, password.as_deref())?;

    // set token
    let token = services().handler.set_token(&username)?;

    // return
    Ok(Json(Response { token }))
}
