// service mod
// this is intermediate abstraction layer for whole project.
// serivce module provide base functionality to api module.
// service module declare trait, database module will impletment these trait.

// sub-module
pub mod message;
pub mod message_state;
pub mod services;
pub mod user;

// this code provide global access function.
use crate::config::Config;
use crate::database::build_database;
use crate::utility::error::Result;
use std::sync::RwLock;

// this is global control, use global instense and rwlock
pub static SERVICES: RwLock<Option<&services::Services>> = RwLock::new(None);

// read, get global SERVICES
pub fn services() -> &'static services::Services {
    SERVICES
        .read()
        .unwrap()
        .expect("SERVICES shoud be initialized when this is called")
}

// write, build Services instense and wirte to SERVICES
pub fn init_service(config: Config) -> Result<()> {
    let db_raw = build_database(config.clone())?;
    let db = Box::leak(db_raw);
    let services_raw = Box::new(services::Services::build(config, db)?);
    *SERVICES.write().unwrap() = Some(Box::leak(services_raw));
    Ok(())
}
// test
#[cfg(test)]
pub mod test {

    use std::fs::{create_dir, remove_dir_all};

    use crate::{
        config::default,
        service::{init_service, services},
    };

    use super::{message::test::test_message, user::tests::test_user};

    #[test]
    fn test_global_services() {
        let mut config = default();
        let tmp_dir = "/tmp/test_services";
        create_dir(tmp_dir.clone()).map_err(|_| {
            remove_dir_all(tmp_dir.clone()).unwrap();
            create_dir(tmp_dir.clone()).unwrap()
        });
        config.database_path = tmp_dir.to_string();
        // set services
        if let Err(e) = init_service(config.clone()) {
            panic!("{}", e);
        }

        // test global
        {
            // get services
            let service_config = &services().config;
            let config = default();
            // check address
            assert_eq!(config.address, service_config.address);
            // check port
            assert_eq!(config.port, service_config.port);
        }

        // test user
        {
            test_user();
        }

        // test message
        {
            test_message();
        }

        // delete
        remove_dir_all(tmp_dir).unwrap();
    }
}
