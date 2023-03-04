use core::fmt;
use std::net::{IpAddr, Ipv4Addr};

use serde::Deserialize;

#[derive(Clone, Debug, Deserialize)]
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
