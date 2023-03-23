use crate::config::Config;

pub struct Services {
    pub config: Config,
}

impl Services {
    pub fn build(config: Config) -> Self {
        Services { config }
    }
}
