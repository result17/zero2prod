use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Router};

pub async fn run() -> Result<(), hyper::Error> {
    let app = Router::new()
        .route("/", get(greet))
        .route("/:name", get(greet))
        .route("/health_check", get(health_check));

    let addr = std::net::SocketAddr::from(([0, 0, 0, 0], 8330));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
}

fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

async fn greet(name: Option<Path<String>>) -> impl IntoResponse {
    if let Some(name) = name {
        return format!(
            "Hello {}!\n",
            capitalize_first_letter(name.to_string().as_str())
        );
    }
    return "Hello world!\n".to_string();
}

async fn health_check() -> impl IntoResponse {
    StatusCode::OK
}
