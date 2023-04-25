use axum::{extract::Path, response::IntoResponse};

fn capitalize_first_letter(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[tracing::instrument(name = "Greet", level = "info")]
pub async fn greet(name: Option<Path<String>>) -> impl IntoResponse {
    if let Some(name) = name {
        return format!(
            "Hello {}!\n",
            capitalize_first_letter(name.to_string().as_str())
        );
    }
    return "Hello world!\n".to_string();
}
