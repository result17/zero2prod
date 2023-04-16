use sqlx::{postgres::{PgPool, PgConnectOptions}, ConnectOptions, Executor};
use std::net::{IpAddr, TcpListener};
use zero2prod::{startup::{get_db_pool, run}, configuration::{get_configuration, Settings}};
use uuid::Uuid;

pub struct TestApp {
    pub port: u16,
    pub address: IpAddr,
    pub db_pool: PgPool,
}

pub async fn spawn_app() -> TestApp {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let address = listener.local_addr().unwrap().ip();
    let mut settings = get_configuration().expect("Failed to get configuration!");
    settings.database.database_name = Uuid::new_v4().to_string();
    println!("Database Name is {}." , settings.database.database_name);
    let db_pool = config_db_pool(&settings).await;
    let test_db_pool = db_pool.clone();
    tokio::spawn(run(listener, db_pool));
    TestApp {
        port,
        address,
        db_pool: test_db_pool,
    }
}

async fn config_db_pool(settings: &Settings) -> PgPool {
    println!("postgres://{}:{}@{}:{}/{}", &settings.database.host, settings.database.port, &settings.database.username, &settings.database.password, &settings.database.database_name);
    let mut connection  = PgConnectOptions::new()
    .host(&settings.database.host)
    .port(settings.database.port)
    .username(&settings.database.username)
    .password(&settings.database.password)
    // .ssl_mode(PgSslMode::Disable)
    .connect().await.expect("Failed to connect postgres");

    connection.execute(format!(r#"CREATE DATABASE "{}";"#, &settings.database.database_name).as_str()).await.expect("Failed to create dateabase");

    let db_pool = get_db_pool(settings).await;
    sqlx::migrate!("./migrations")
    .run(&db_pool)
    .await
    .expect("Failed to migrate the database");

    db_pool
}  
