use axum::Json;
use axum_auth::AuthBearer;
use serde::Serialize;

use crate::{api::get_user_from_token, service::services, utility::error::Result};

#[derive(Serialize)]
pub struct Response {
    users: Vec<String>,
}

pub async fn get_users(AuthBearer(token): AuthBearer) -> Result<Json<Response>> {
    tracing::info!("token: {:?}", token);
    let user = get_user_from_token(token)?;

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
