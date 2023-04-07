use axum::{http::StatusCode, response::IntoResponse};
use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    BadConfig(&'static str),

    #[error("{0}")]
    BadDatabase(&'static str),

    #[error("{0}")]
    BadRequest(&'static str),

    #[cfg(feature = "sqlite")]
    #[error("There was a problem with the connection to the sqlite database: {source}")]
    SqliteError {
        #[from]
        source: rusqlite::Error,
    },
}

impl Error {
    pub fn bad_config(msg: &'static str) -> Self {
        Self::BadConfig(msg)
    }

    pub fn bad_database(msg: &'static str) -> Self {
        Self::BadDatabase(msg)
    }
}

impl IntoResponse for Error {
    fn into_response(self) -> axum::response::Response {
        tracing::error!("{:?}", self);
        match self {
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
        .into_response()
    }
}
