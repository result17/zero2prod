use crate::routes::{greet, health_check};
use axum::{routing::get, Router, Server};
use std::net::TcpListener;

pub async fn run(listener: TcpListener) -> Result<(), hyper::Error> {
    let app = Router::new()
        .route("/", get(greet))
        .route("/:name", get(greet))
        .route("/health_check", get(health_check));

    Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .await
}
