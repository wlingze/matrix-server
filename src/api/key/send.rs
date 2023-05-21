use axum::Json;
use axum_auth::AuthBearer;

use serde::Deserialize;

use crate::{api::get_user_from_token, service::services, utility::error::Result};

#[derive(Deserialize)]
pub struct Body {
    pub public_key: Vec<u8>,
}

pub async fn send_key(
    AuthBearer(token): AuthBearer,
    Json(Body { public_key }): Json<Body>,
) -> Result<()> {
    services()
        .handler
        .set_key(get_user_from_token(token)?.as_str(), public_key)?;
    Ok(())
}
