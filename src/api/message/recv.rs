use axum::Json;
use axum_auth::AuthBearer;
use serde::{Deserialize, Serialize};

use crate::{
    api::get_user_from_token,
    service::{message::Message, services},
    utility::error::{Error, Result},
};

#[derive(Deserialize)]
pub struct Body {
    since: String,
}

#[derive(Serialize)]
pub struct Response {
    next_since: String,
    messages: Vec<Message>,
}

/// # `POST /api/v0/sync`
///
///  send message
pub async fn recv(
    AuthBearer(token): AuthBearer,
    Json(Body { since }): Json<Body>,
) -> Result<Json<Response>> {
    tracing::info!("token: {:?}, since: {:?}, ", token, since);

    // parse UserId
    let user = get_user_from_token(token)?;
    tracing::info!("user: {:?}", user);

    let (messages, next_since) = services()
        .handler
        .recv_message(&user, &since)?
        .ok_or(Error::BadRequest("Wrong since."))?;
    tracing::info!("tuple: {:?}", next_since);

    if (next_since != since && messages.len() == 0) || (next_since == since && messages.len() != 0)
    {
        tracing::error!("recv error");
        let tuple = services()
            .handler
            .recv_message(&user, "0")?
            .ok_or(Error::BadRequest("Wrong since."))?;
        for msg in tuple.0 {
            tracing::error!("{:?}", msg)
        }
        tracing::error!("since 1");
        let tuple = services()
            .handler
            .recv_message(&user, "1")?
            .ok_or(Error::BadRequest("Wrong since."))?;
        for msg in tuple.0 {
            tracing::error!("{:?}", msg)
        }
    }

    Ok(Json(Response {
        next_since,
        messages,
    }))
}
