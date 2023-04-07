use axum::Json;
use serde::{Deserialize, Serialize};

use crate::{
    service::{message::Message, services},
    utility::error::{Error, Result},
};

#[derive(Deserialize)]
pub struct Body {
    token: String,
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
pub async fn recv(Json(body): Json<Body>) -> Result<Json<Response>> {
    tracing::info!("token: {:?}, since: {:?}, ", body.token, body.since);

    // parse UserId
    let user = services()
        .handler
        .from_token(body.token)?
        .ok_or(Error::BadRequest("Wrong token."))?;
    tracing::debug!("user: {:?}", user);

    let tuple = services()
        .handler
        .recv_message(&user, &body.since)?
        .ok_or(Error::BadRequest("Wrong since."))?;
    tracing::debug!("tuple: {:?}", tuple);

    Ok(Json(Response {
        next_since: tuple.1,
        messages: tuple.0,
    }))
}
