mod api;
mod config;
mod database;
mod service;
mod utility;

use std::{io, net::SocketAddr};

use axum::{routing::get, Router};
use axum_server::{bind, Handle};
use service::{init_service, services};

#[tokio::main]
async fn main() {
    let config = config::parse();

    // set config to service
    if let Err(e) = init_service(config) {
        eprintln!("It looks like your config is invalid: {}", e);
        std::process::exit(-1);
    };

    // start web server
    let start = async {
        run_server().await.unwrap();
    };
    start.await;
}

async fn run_server() -> io::Result<()> {
    // get config from service
    let config = &services().config;

    // start web server
    let addr = SocketAddr::from((config.address, config.port));
    let hander = Handle::new();
    let app = routes().into_make_service();
    bind(addr).handle(hander).serve(app).await?;

    Ok(())
}

fn routes() -> Router {
    Router::new().route("/ping", get(api::ping))
}
