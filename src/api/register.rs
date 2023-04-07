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
pub async fn register_route(Json(body): Json<Body>) -> Result<Json<Response>> {
    // create user
    // set user-password
    services()
        .handler
        .set_password(&body.username, body.password.as_deref())?;

    // set token
    let token = services().handler.set_token(&body.username)?;

    // return
    Ok(Json(Response { token }))
}
