use axum::{Router, routing::post, Json};
use serde::Deserialize;

#[derive(Deserialize)]
struct WebhookEvent {
    event: String,
    data: serde_json::Value,
}

async fn webhook_handler(Json(payload): Json<WebhookEvent>) -> &'static str {
    println!("Received webhook event: {:?}", payload);
    "ok"
}

pub fn router() -> Router {
    Router::new()
        .route("/webhook", post(webhook_handler))
}
