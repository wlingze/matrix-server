use axum::Json;
use axum_auth::AuthBearer;
use serde::Deserialize;

use crate::{api::token_check, utility::error::Result};

#[derive(Deserialize)]
pub struct Body {
    name: String,
}

pub async fn check_token(
    AuthBearer(token): AuthBearer,
    Json(Body { name }): Json<Body>,
) -> Result<()> {
    // check user by token
    tracing::info!("token: {:?}", token);
    token_check(token, |str| str == name)
}
