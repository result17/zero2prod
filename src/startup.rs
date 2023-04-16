use crate::configuration::Settings;
use crate::routes::{greet, health_check, subscriptions};
use axum::{
    routing::{get, post},
    Router, Server,
};
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::{net::TcpListener, time::Duration};

pub async fn run(listener: TcpListener, db_pool: PgPool) -> Result<(), hyper::Error> {
    let app = Router::new()
        .route("/greet", get(greet))
        .route("/greet/:name", get(greet))
        .route("/health_check", get(health_check))
        .route("/subscriptions", post(subscriptions))
        .with_state(db_pool);

    Server::from_tcp(listener)?
        .serve(app.into_make_service())
        .await
}

pub async fn get_db_pool(settings: &Settings) -> PgPool {
    println!("Database name is {} now!", &settings.database.database_name);
    println!("Connecton_string: {}", &settings.database.connection_string());
    PgPoolOptions::new()
        .max_connections(settings.database.max_connections)
        .acquire_timeout(Duration::from_millis(settings.database.connect_timeout))
        .connect(&settings.database.connection_string())
        .await
        .expect("Failed to connect postgres!")
}
