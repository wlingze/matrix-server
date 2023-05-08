use axum::Json;
use serde::{Deserialize, Serialize};

use crate::{
    service::services,
    utility::error::{Error, Result},
};

#[derive(Deserialize)]
pub struct Body {
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct Response {
    pub token: String,
}
/// # `POST /api/v0/login`
///
/// login in this server
///
pub async fn login_route(Json(Body { username, password }): Json<Body>) -> Result<Json<Response>> {
    // get password and check user exists
    let hash = services()
        .handler
        .get_password(&username)?
        .ok_or(Error::BadRequest("Wrong username."))?;

    // check password
    if !argon2::verify_encoded(&hash, password.as_bytes()).unwrap_or(false) {
        return Err(Error::BadRequest("Wrong password."));
    }

    // return
    Ok(Json(Response {
        token: services().handler.set_token(&username)?,
    }))
}
