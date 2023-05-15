use axum::{extract::State, http::StatusCode, response::IntoResponse, Form};
use chrono::Utc;
use serde::Deserialize;
use sqlx::{postgres::PgPool, query};
use uuid::Uuid;
use tracing::{info, info_span};

#[derive(Deserialize)]
pub struct User {
    pub email: String,
    pub name: String,
}

pub async fn subscriptions(
    State(state): State<PgPool>,
    Form(user): Form<User>,
) -> impl IntoResponse {
    let request_id = Uuid::new_v4();
    let request_span = info_span!(
        "Adding a new subscriber.",
        %request_id,
        subscriber_email = %user.email,
        subscriber_name = %user.name
    );
    let _request_span_guard = request_span.enter();
    

    match query!(
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
    .await
    {
        Ok(_) => {
            info!("New subscriber details have been saved");
            StatusCode::OK
        },
        Err(e) => {
            println!("Failed to execute query: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
