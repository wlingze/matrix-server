use thiserror::Error;

pub type Result<T, E = Error> = std::result::Result<T, E>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("{0}")]
    BadConfig(&'static str),
}

impl Error {
    pub fn bad_config(msg: &'static str) -> Self {
        Self::BadConfig(msg)
    }
}
