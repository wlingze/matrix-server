use core::fmt;
use std::net::{IpAddr, Ipv4Addr};

use figment::{
    providers::{Env, Format, Toml},
    Figment,
};
use serde::Deserialize;

// config pares from config file `xxx.toml` by figment
pub fn parse() -> Config {
    let raw_config = Figment::new()
        .merge(
            Toml::file(
                Env::var("MATRIX_CONFIG")
                    .expect("The MATRIX_CONFIG env var needs to be set. Example: /etc/matrix.toml"),
            )
            .nested(),
        )
        .merge(Env::prefixed("MATRIX_").global());

    match raw_config.extract::<Config>() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("it look like your config is invaild: {}", e);
            std::process::exit(-1);
        }
    }
}
// this struct containing config data
#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    // http server address
    #[serde(default = "default_address")]
    pub address: IpAddr,
    #[serde(default = "default_port")]
    pub port: u16,

    // database
    #[serde(default = "default_database_backend")]
    pub database_backend: String,
    #[serde(default = "default_database_path")]
    pub database_path: String,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lines = [
            ("Server address", self.address.to_string()),
            ("Server port", self.port.to_string()),
            ("Database backend", self.database_backend.to_string()),
            ("Database path", self.database_path.to_string()),
        ];

        let mut msg = "".to_string();
        for line in lines.into_iter().enumerate() {
            msg += &format!("{}: {}\n", line.1 .0, line.1 .1);
        }

        write!(f, "{}", msg)
    }
}

fn default_address() -> IpAddr {
    Ipv4Addr::LOCALHOST.into()
}

fn default_port() -> u16 {
    8000
}

fn default_database_backend() -> String {
    "sqlite".to_string()
}
fn default_database_path() -> String {
    "".to_string()
}

#[cfg(test)]
mod test {
    use std::net::Ipv4Addr;

    use figment::{
        providers::{Env, Format, Toml},
        Figment,
    };

    use crate::config::{default_address, default_database_backend, default_port, Config};

    #[test]
    fn test_config_parse() {
        let config_file_name = "Test.toml";
        let figment = || {
            Figment::new()
                .merge(Toml::file(config_file_name))
                .merge(Env::prefixed("MATRIX_").global())
        };

        figment::Jail::expect_with(|jail| {
            // check default
            {
                let test: Config = figment().extract()?;
                assert_eq!(test.address, default_address());
                assert_eq!(test.port, default_port());
                assert_eq!(test.database_backend, default_database_backend());
                assert_eq!(test.database_path, "".to_string());
            }

            // check toml file
            {
                jail.create_file(
                    config_file_name,
                    r#"
                port = 1234
                address = "127.1.1.1"
                database_backend = "sqlite1"
                database_path = "/tmp"
                "#,
                )?;
                let test: Config = figment().extract()?;
                assert_eq!(test.port, 1234);
                assert_eq!(test.address, Ipv4Addr::from([127, 1, 1, 1]));
                assert_eq!(test.database_backend, "sqlite1".to_string());
                assert_eq!(test.database_path, "/tmp".to_string());
            }

            // check environment
            {
                jail.set_env("MATRIX_ADDRESS", "127.2.2.2");
                jail.set_env("MATRIX_PORT", "2345");
                jail.set_env("MATRIX_DATABASE_BACKEND", "sqlite2");
                jail.set_env("MATRIX_DATABASE_PATH", "/tmp2");
                let test: Config = figment().extract()?;
                assert_eq!(test.port, 2345);
                assert_eq!(test.address, Ipv4Addr::from([127, 2, 2, 2]));
                assert_eq!(test.database_backend, "sqlite2".to_string());
                assert_eq!(test.database_path, "/tmp2".to_string());
            }
            Ok(())
        })
    }
}
