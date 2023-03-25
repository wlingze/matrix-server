use crate::{config::Config, utility::error::Result};

use crate::service::{global, user};

pub struct Services {
    pub global: global::Services,
    pub user: user::Services,
}

impl Services {
    // build a Services instance
    pub fn build<H>(config: Config, handler: &'static H) -> Result<Self>
    where
        H: user::Handler + 'static,
    {
        Ok(Services {
            global: global::Services::build(config),
            user: user::Services { handler },
        })
    }
}
