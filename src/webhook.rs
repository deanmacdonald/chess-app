use axum::{
    extract::{Json, State},
    routing::post,
    Router,
};
use chrono::Utc;
use hmac::{Hmac, KeyInit, Mac};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sha2::Sha256;
use std::sync::Arc;
use tokio::sync::RwLock;

use crate::state::AppState;

type SharedState = Arc<RwLock<AppState>>;

// -------------------------------
// Webhook Payload
// -------------------------------
#[derive(Deserialize, Serialize)]
pub struct WebhookEvent {
    pub event: String,
    pub data: Value,
    pub timestamp: i64,
    pub signature: String,
}

// -------------------------------
// HMAC Verification
// -------------------------------
fn verify_signature(secret: &str, body: &str, signature: &str) -> bool {
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(body.as_bytes());

    match mac.verify_slice(signature.as_bytes()) {
        Ok(_) => true,
        Err(_) => false,
    }
}

// -------------------------------
// Replay Protection
// -------------------------------
fn is_recent(timestamp: i64) -> bool {
    let now = Utc::now().timestamp();
    let diff = (now - timestamp).abs();
    diff <= 30 // 30-second replay window
}

// -------------------------------
// Event Dispatcher
// -------------------------------
async fn handle_event(state: &SharedState, event: &str, data: &Value) {
    match event {
        "load_fen" => {
            if let Some(fen) = data.get("fen").and_then(|v| v.as_str()) {
                let mut s = state.write().await;
                s.engine.load_fen(fen);
                println!("Webhook: Loaded FEN {}", fen);
            }
        }

        "reset" => {
            let mut s = state.write().await;
            s.engine.reset();
            println!("Webhook: Reset game");
        }

        "apply_move" => {
            if let (Some(from), Some(to)) = (
                data.get("from").and_then(|v| v.as_str()),
                data.get("to").and_then(|v| v.as_str()),
            ) {
                let mut s = state.write().await;
                let (fen, id, result) = s.engine.make_move_strict(from, to);
                println!("Webhook: Move {}{} → {} (id {})", from, to, result, id);
                println!("New FEN: {}", fen);
            }
        }

        _ => {
            println!("Webhook: Unknown event '{}'", event);
        }
    }
}

// -------------------------------
// Main Handler
// -------------------------------
async fn webhook_handler(
    State(state): State<SharedState>,
    Json(payload): Json<WebhookEvent>,
) -> &'static str {
    let secret = std::env::var("WEBHOOK_SECRET").unwrap_or_else(|_| "dev-secret".to_string());

    // Convert payload back to JSON string for HMAC
    let body = serde_json::to_string(&payload).unwrap();

    // Verify signature
    if !verify_signature(&secret, &body, &payload.signature) {
        println!("Webhook: Invalid signature");
        return "invalid signature";
    }

    // Replay protection
    if !is_recent(payload.timestamp) {
        println!("Webhook: Replay attack detected");
        return "expired";
    }

    // Dispatch event
    handle_event(&state, &payload.event, &payload.data).await;

    "ok"
}

// -------------------------------
// Router
// -------------------------------
pub fn router() -> Router<SharedState> {
    Router::new().route("/webhook", post(webhook_handler))
}
