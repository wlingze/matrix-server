use axum::Json;
use axum_auth::AuthBearer;
use serde::{Deserialize, Serialize};

use crate::{
    api::token_check,
    service::services,
    utility::error::{Error, Result},
};

#[derive(Deserialize)]
pub struct Body {
    username: String,
}

#[derive(Serialize)]
pub struct Response {
    public_key: Vec<u8>,
}

pub async fn get_key(
    AuthBearer(token): AuthBearer,
    Json(Body { username }): Json<Body>,
) -> Result<Json<Response>> {
    token_check(token, |str| str != "".to_string())?;

    Ok(Json(Response {
        public_key: services()
            .handler
            .get_key(&username)?
            .ok_or(Error::BadRequest("Wrong username."))?,
    }))
}
