use zero2prod::run;

#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    run().await
}
