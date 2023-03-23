use crate::{config::Config, utility::error::Result};

pub fn build_database(_config: Config) -> Result<Database> {
    Ok(Database {})
}

pub struct Database {
    // todo
}
