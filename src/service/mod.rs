// service mod
// provide database and global info

use crate::config::Config;
use crate::utility::error::Result;
use std::sync::RwLock;

// this is global control, use global instense and rwlock
pub static SERVICES: RwLock<Option<&Services>> = RwLock::new(None);

// read, get global SERVICES
pub fn services() -> &'static Services {
    SERVICES
        .read()
        .unwrap()
        .expect("SERVICES shoud be initialized when this is called")
}

// write, build Services instense and wirte to SERVICES
pub fn init_service(config: Config) -> Result<()> {
    let services_raw = Box::new(Services::build(config)?);
    *SERVICES.write().unwrap() = Some(Box::leak(services_raw));
    Ok(())
}

pub struct Services {
    pub config: Config,
}

impl Services {
    // build a Services instance
    fn build(config: Config) -> Result<Self> {
        Ok(Self { config })
    }
}
