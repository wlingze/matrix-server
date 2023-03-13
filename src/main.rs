mod api;
mod config;
mod service;
mod utility;

use std::{io, net::SocketAddr};

use axum::{routing::get, Router};
use axum_server::{bind, Handle};
use config::Config;

#[tokio::main]
async fn main() {
    let config = config::parse();

    // set config to service

    // start web server
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
