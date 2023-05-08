use axum::Json;
use axum_auth::AuthBearer;
use serde::Deserialize;

use crate::{
    api::get_user_from_token,
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
pub async fn send(AuthBearer(token): AuthBearer, Json(Body { message }): Json<Body>) -> Result<()> {
    tracing::info!("token: {:?}, message: {:?}", token, message);

    // check user by token
    let user = get_user_from_token(token)?;

    tracing::debug!("user-from-token: {:?}, message: {:?}", user, message);
    if user == message.send {
        services().handler.send_message(message)?;
        Ok(())
    } else {
        Err(Error::BadRequest("Wrong user."))
    }
}
