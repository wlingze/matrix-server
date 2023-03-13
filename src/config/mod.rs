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
#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(default = "default_address")]
    pub address: IpAddr,
    #[serde(default = "default_port")]
    pub port: u16,
}

impl fmt::Display for Config {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let lines = [
            ("Server address", self.address.to_string()),
            ("Server port", self.port.to_string()),
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
