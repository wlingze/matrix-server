use crate::{config::Config, utility::error::Result};

use super::message_state::MessageState;
use super::{key, message, user};

pub trait Handler: user::Handler + message::Handler + key::Handler {}

pub struct Services {
    pub config: Config,
    pub state: MessageState,
    pub handler: &'static dyn Handler,
}

impl Services {
    // build a Services instance
    pub fn build<H>(config: Config, handler: &'static H) -> Result<Self>
    where
        H: Handler + 'static,
    {
        Ok(Services {
            config,
            handler,
            state: MessageState::new(handler),
        })
    }
}
