use crate::helpers::spawn_app;
use sqlx::{query};

#[tokio::test]
async fn subscribe_returns_a_200_for_vaild_form_data() {
    let app = spawn_app().await;
    let socket_addr = format!("http://{}:{}", app.address, app.port);
    let client = reqwest::Client::new();
    println!("Socket addr {}", socket_addr);
    let body = "name=le%20guin&email=ursula_le_guin%40gmail.com";
    let response = client
        .post(&format!("{}/subscriptions", &socket_addr))
        .header("Content-Type", "application/x-www-form-urlencoded")
        .body(body)
        .send()
        .await
        .expect("Failed to execute request.");

    assert_eq!(200, response.status().as_u16());

    let saved_email = "ursula_le_guin@gmail.com";
    let saved_name = "le guin";
    let saved = query!(
        "SELECT email, name FROM subscriptions WHERE email = ($1) and name = ($2)",
        saved_email,
        saved_name
    )
    .fetch_one(&app.db_pool)
    .await
    .expect("Failed to fetch saved subscription");

    assert_eq!(saved.email, saved_email);
    assert_eq!(saved.name, saved_name);
}

#[tokio::test]
async fn subscribe_returns_a_422_when_data_is_missing() {
    let app = spawn_app().await;
    let socket_addr = format!("http://{}:{}", app.address, app.port);
    let client = reqwest::Client::new();
    let test_cases = vec![
        ("name=le%20guin", "missing the email"),
        ("email=ursula_le_guin%40gmail.com", "missing the name"),
        ("", "missing both name and email"),
    ];

    for (invalid_body, error_message) in test_cases {
        let response = client
            .post(&format!("{}/subscriptions", &socket_addr))
            .header("Content-Type", "application/x-www-form-urlencoded")
            .body(invalid_body)
            .send()
            .await
            .expect("Failed to execute request.");

        assert_eq!(
            422,
            response.status().as_u16(),
            "The API did not fail with 422 Bad Request when the payload was {}.",
            error_message
        )
    }
}
