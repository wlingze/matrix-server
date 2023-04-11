use axum::Json;
use axum_auth::AuthBearer;
use serde::Serialize;

use crate::{
    service::services,
    utility::error::{Error, Result},
};

#[derive(Serialize)]
pub struct Response {
    users: Vec<String>,
}

pub async fn get_users(AuthBearer(token): AuthBearer) -> Result<Json<Response>> {
    services()
        .handler
        .from_token(token)?
        .ok_or(Error::BadRequest("Wrong token."))?;

    Ok(Json(Response {
        users: services().handler.users()?,
    }))
}
