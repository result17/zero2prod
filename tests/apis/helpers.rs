use sqlx::postgres::PgPool;
use std::net::{IpAddr, TcpListener};
use zero2prod::startup::{get_db_pool, run};

pub struct TestApp {
    pub port: u16,
    pub address: IpAddr,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = listener.local_addr().unwrap().ip();
    let db_pool = get_db_pool().await;
    tokio::spawn(run(listener));
    TestApp {
        port,
        address,
        db_pool,
    }
}
