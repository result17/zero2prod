use axum::{
    extract::Path,
    http::StatusCode,
    response::{IntoResponse},
    routing::{get, post},
    Json, Router,
};
use serde::Serialize;
use std::net::TcpListener;
use tower_http::cors::{CorsLayer, Any};

pub async fn run(listener: TcpListener) -> Result<(), hyper::Error> {
    let app = Router::new()
        .route("/", get(greet))
        .route("/:name", get(greet))
        .route("/health_check", get(health_check))
        .route("/options", post(metric_options).layer(
            CorsLayer::new().allow_origin(Any).allow_headers(Any)
        ));

    axum::Server::from_tcp(listener)?
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

#[derive(Serialize)]
struct Opt {
    value: String,
    count: u32,
    alias: String,
}

#[derive(Serialize)]
struct OptRecord {
    options: Vec<Opt>
}

#[derive(Serialize)]
struct OptResponse {
    code: u16,
    data: OptRecord,
    msg: String,
}

async fn metric_options() -> impl IntoResponse {
    let opt = Opt {
        value: "cn".to_string(),
        count: 100,
        alias: "中国".to_string(),
    };
    let opt_record = OptRecord {
        options: vec![opt]
    };
    let response = OptResponse {
        code: 200,
        data: opt_record,
        msg: "mock".to_string(),
    };
    Json(response)
}
