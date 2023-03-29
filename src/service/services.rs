use crate::{config::Config, utility::error::Result};

use crate::service::user;

pub trait Handler: user::Handler {}

// test only check config
#[cfg(test)]
pub struct Services {
    pub config: Config,
}
#[cfg(test)]
impl Services {
    // build a Services instance
    pub fn build(config: Config) -> Result<Self> {
        Ok(Services { config })
    }
}

#[cfg(not(test))]
pub struct Services {
    pub config: Config,
    pub handler: &'static dyn Handler,
}

#[cfg(not(test))]
impl Services {
    // build a Services instance
    pub fn build<H>(config: Config, handler: &'static H) -> Result<Self>
    where
        H: Handler + 'static,
    {
        Ok(Services { config, handler })
    }
}
