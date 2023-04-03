use crate::routes::{greet, health_check, subscriptions};
use axum::{
    routing::{get, post},
    Router, Server,
};
use std::net::TcpListener;

pub async fn run(listener: TcpListener) -> Result<(), hyper::Error> {
    let app = Router::new()
        .route("/greet", get(greet))
        .route("/greet/:name", get(greet))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscriptions));

    Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .await
}
