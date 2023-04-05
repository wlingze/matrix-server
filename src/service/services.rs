use crate::{config::Config, utility::error::Result};

use crate::service::user;

use super::message;

pub trait Handler: user::Handler + message::Handler {}

pub struct Services {
    pub config: Config,
    pub handler: &'static dyn Handler,
}

impl Services {
    // build a Services instance
    pub fn build<H>(config: Config, handler: &'static H) -> Result<Self>
    where
        H: Handler + 'static,
    {
        Ok(Services { config, handler })
    }
}
