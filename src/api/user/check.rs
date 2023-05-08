use axum::Json;
use axum_auth::AuthBearer;
use serde::Deserialize;

use crate::{service::services, utility::error::Result};

#[derive(Deserialize)]
pub struct Body {
    name: String,
}

pub async fn check_token(
    AuthBearer(token): AuthBearer,
    Json(Body { name }): Json<Body>,
) -> Result<Json<bool>> {
    // check user by token
    tracing::info!("token: {:?}", token);
    Ok(Json(
        services()
            .handler
            .from_token(token)?
            .map(|user| user == name)
            .unwrap_or(false),
    ))
}
