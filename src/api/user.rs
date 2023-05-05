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
    tracing::info!("token: {:?}", token);
    let user = services()
        .handler
        .from_token(token)?
        .ok_or(Error::BadRequest("Wrong token."))?;

    tracing::debug!("token check ok");

    Ok(Json(Response {
        users: services()
            .handler
            // get all user
            .users()?
            // without current user
            .into_iter()
            .filter(|name| name != &user)
            .collect(),
    }))
}
