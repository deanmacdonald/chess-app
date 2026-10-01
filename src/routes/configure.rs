use axum::{Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct ConfigureResponse {
    pub app_name: String,
    pub version: String,
    pub websocket_url: String,
    pub allow_guests: bool,
}

pub async fn configure_handler() -> Json<ConfigureResponse> {
    Json(ConfigureResponse {
        app_name: "chess-app".to_string(),
        version: "1.0.0".to_string(),
        websocket_url: "/ws".to_string(),
        allow_guests: true,
    })
}
