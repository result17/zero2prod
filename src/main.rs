use std::net::TcpListener;
use zero2prod::{
    startup::{run, get_db_pool},
    configuration::get_configuration,
    telemetry::{ get_subscriber, init_subscriber },
};

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    println!("Listening on http://127.0.0.1:{}", port);
    let pool = get_db_pool(&get_configuration().expect("Failed to get configuration!")).await;
    
    let subscriber = get_subscriber("zero2prod", "info", std::io::stdout);
    init_subscriber(subscriber);
    
    run(listener, pool).await
}
