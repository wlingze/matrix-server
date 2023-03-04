mod api;
mod config;

use std::{io, net::SocketAddr};

use axum::{routing::get, Router};
use axum_server::{bind, Handle};
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
    let start = async {
        run_server(config).await.unwrap();
    };

    start.await;
}

async fn run_server(config: Config) -> io::Result<()> {
    let addr = SocketAddr::from((config.address, config.port));
    let hander = Handle::new();
    let app = routes().into_make_service();
    bind(addr).handle(hander).serve(app).await?;

    Ok(())
}

fn routes() -> Router {
    Router::new().route("/ping", get(api::ping))
}
