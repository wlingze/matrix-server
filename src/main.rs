mod api;
mod config;
mod database;
mod service;
mod utility;

use std::{io, net::SocketAddr};

use axum::{
    extract::MatchedPath,
    http::{self, header, HeaderName, Method},
    routing::{get, post},
    Router,
};
use axum_server::{bind, Handle};
use service::{init_service, services};
use tower::ServiceBuilder;
use tower_http::{
    cors::{self, CorsLayer},
    trace::TraceLayer,
};

#[tokio::main]
async fn main() {
    // set tracing log
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let config = config::parse();

    tracing::info!("config: {:?}", config);

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

    let x_requested_with = HeaderName::from_static("x-requested-with");

    let middleware = ServiceBuilder::new()
        .layer(
            TraceLayer::new_for_http().make_span_with(|request: &http::Request<_>| {
                let path = if let Some(path) = request.extensions().get::<MatchedPath>() {
                    path.as_str()
                } else {
                    request.uri().path()
                };
                tracing::info_span!("http_request", %path)
            }),
        )
        .layer(
            CorsLayer::new()
                .allow_origin(cors::Any)
                .allow_methods([Method::POST, Method::GET])
                // .allow_headers([header::ORIGIN, header::ACCEPT, header::AUTHORIZATION]),
                .allow_headers([
                    header::ORIGIN,
                    x_requested_with,
                    header::CONTENT_TYPE,
                    header::ACCEPT,
                    header::AUTHORIZATION,
                ]),
        );

    let app = routes().layer(middleware).into_make_service();
    bind(addr).handle(hander).serve(app).await?;

    Ok(())
}

fn routes() -> Router {
    Router::new()
        .route("/ping", get(api::ping))
        .route("/api/v0/get_users", get(api::user::users::get_users))
        .route(
            "/api/v0/register",
            post(api::user::register::register_route),
        )
        .route("/api/v0/check", post(api::user::check::check_token))
        .route("/api/v0/login", post(api::user::login::login_route))
        .route("/api/v0/send", post(api::message::send::send))
        .route("/api/v0/recv", post(api::message::recv::recv))
        .route("/api/v0/send_key", post(api::key::send::send_key))
        .route("/api/v0/get_key", post(api::key::get::get_key))
}
