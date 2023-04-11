use axum::Json;
use axum_auth::AuthBearer;
use serde::Deserialize;

use crate::{
    service::{message::Message, services},
    utility::error::{Error, Result},
};

#[derive(Deserialize)]
pub struct Body {
    pub message: Message,
}

/// # `POST /api/v0/send`
///
///  send message
pub async fn send(AuthBearer(token): AuthBearer, Json(body): Json<Body>) -> Result<()> {
    tracing::info!("token: {:?}, message: {:?}", token, body.message);

    // check user by token
    let user = services()
        .handler
        .from_token(token)?
        .ok_or(Error::BadRequest("Wrong token."))?;

    tracing::debug!("user-from-token: {:?}, message: {:?}", user, body.message);
    if user == body.message.send {
        services().handler.send_message(body.message)?;
        Ok(())
    } else {
        Err(Error::BadRequest("Wrong user."))
    }
}
