mod config;

use config::Config;
use figment::{
    providers::{Env, Format, Toml},
    Figment,
};

#[tokio::main]
async fn main() {
    let raw_config = Figment::new()
        .merge(
            Toml::file(
                Env::var("MATRIX_CONFIG")
                    .expect("The MATRIX_CONFIG env var needs to be set. Example: /etc/matrix.toml"),
            )
            .nested(),
        )
        .merge(Env::prefixed("MATRIX_").global());

    let config = match raw_config.extract::<Config>() {
        Ok(s) => s,
        Err(e) => {
            eprintln!("it look like your config is invaild: {}", e);
            std::process::exit(-1);
        }
    };

    // use it config
    println!("{}", config);
}
