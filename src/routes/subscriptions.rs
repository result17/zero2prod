use axum::{http::StatusCode, response::IntoResponse, Form};
use serde::Deserialize;
#[derive(Deserialize)]
pub struct User {
    pub email: String,
    pub name: String,
}
pub async fn subscriptions(Form(_user): Form<User>) -> impl IntoResponse {
    StatusCode::BAD_REQUEST
}
