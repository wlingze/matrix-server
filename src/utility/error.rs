use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    BadConfig(&'static str),

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
}
