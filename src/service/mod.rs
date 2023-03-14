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

#[cfg(test)]
mod test {
    use std::net::Ipv4Addr;

    use crate::{
        config::Config,
        service::{init_service, services},
    };

    #[test]
    fn test_global_services() {
        println!("test ");
        let config = Config {
            address: Ipv4Addr::LOCALHOST.into(),
            port: 8000,
        };

        // set services
        match init_service(config.clone()) {
            Ok(_) => {}
            Err(e) => {
                panic!("{}", e)
            }
        };

        // get services
        let service_config = &services().config;

        // check address
        assert_eq!(config.address, service_config.address);
        // check port
        assert_eq!(config.port, service_config.port);
    }
}
