use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::RwLock;

mod engine;
mod routes;
mod state;
mod webhook;

use crate::routes::configure::build_routes;
use state::AppState;

#[tokio::main]
async fn main() {
    let state = Arc::new(RwLock::new(AppState::new()));
    let app = build_routes(state);

    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let address = format!("0.0.0.0:{port}");

    let listener = TcpListener::bind(&address)
        .await
        .expect("failed to bind server");

    println!("Chess backend running at http://{address}");

    axum::serve(listener, app).await.expect("server error");
}
