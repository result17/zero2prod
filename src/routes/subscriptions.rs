use axum::{extract::State, http::StatusCode, response::IntoResponse, Form};
use chrono::Utc;
use serde::Deserialize;
use sqlx::{postgres::PgPool, query};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct User {
    pub email: String,
    pub name: String,
}

pub async fn subscriptions(
    Form(user): Form<User>,
    State(state): State<PgPool>,
) -> impl IntoResponse {
    query!(
        "
        INSERT INTO subscriptions (id, email, name, subscribed_at)
        VALUES ($1, $2, $3, $4)
        ",
        Uuid::new_v4(),
        user.email,
        user.name,
        Utc::now(),
    )
    .execute(&state)
    .await;
    StatusCode::BAD_REQUEST
}
