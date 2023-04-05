use crate::configuration::get_configuration;
use crate::routes::{greet, health_check, subscriptions};
use axum::{
    routing::{get, post},
    Router, Server,
};
use sqlx::postgres::PgPoolOptions;
use std::{net::TcpListener, time::Duration};

pub async fn run(listener: TcpListener) -> Result<(), hyper::Error> {
    let config = get_configuration().expect("Failed to get configuration!");
    let dbSettings = config.database;
    let pool = PgPoolOptions::new()
        .max_connections(dbSettings.max_connections)
        .acquire_timeout(Duration::from_millis(dbSettings.connect_timeout))
        .connect(&dbSettings.connection_string())
        .await
        .expect("Failed to connect postgres!");
    let app = Router::new()
        .route("/greet", get(greet))
        .route("/greet/:name", get(greet))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscriptions))
        .with_state(pool);

    Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .await
}
