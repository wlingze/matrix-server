use crate::{config::Config, utility::error::Result};

use crate::service::global;

pub struct Services {
    pub global: global::Services,
}

impl Services {
    // build a Services instance
    pub fn build<H>(config: Config, _handler: H) -> Result<Self> {
        Ok(Services {
            global: global::Services::build(config),
        })
    }
}
