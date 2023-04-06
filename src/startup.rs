use crate::configuration::get_configuration;
use crate::routes::{greet, health_check, subscriptions};
use axum::{
    routing::{get, post},
    Router, Server,
};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::{net::TcpListener, time::Duration};

pub async fn run(listener: TcpListener) -> Result<(), hyper::Error> {
    let pool = get_db_pool().await;
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

pub async fn get_db_pool() -> PgPool {
    let config = get_configuration().expect("Failed to get configuration!");
    let db_settings = config.database;
    PgPoolOptions::new()
        .max_connections(db_settings.max_connections)
        .acquire_timeout(Duration::from_millis(db_settings.connect_timeout))
        .connect(&db_settings.connection_string())
        .await
        .expect("Failed to connect postgres!")
}
