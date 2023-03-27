use std::net::TcpListener;
use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    println!("Listening on http://127.0.0.1:{}", port);
    run(listener).await
}
