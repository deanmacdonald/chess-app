use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::IntoResponse;
use std::sync::{Arc, Mutex};

/// Shared broadcast channels for all websocket clients
#[derive(Clone)]
pub struct WsChannels {
    pub clients: Arc<Mutex<Vec<WebSocket>>>,
}

impl WsChannels {
    pub fn new() -> Self {
        Self {
            clients: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Broadcast a text message to all connected clients
    pub async fn broadcast(&self, text: &str) {
        let mut dead = vec![];

        let mut clients = self.clients.lock().unwrap();
        for (i, client) in clients.iter_mut().enumerate() {
            if client.send(Message::Text(text.to_string())).await.is_err() {
                dead.push(i);
            }
        }

        // Remove dead sockets
        for i in dead.into_iter().rev() {
            clients.remove(i);
        }
    }
}

/// WebSocket upgrade handler
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(channels): State<WsChannels>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, channels))
}

/// Handle a single websocket connection
async fn handle_socket(mut socket: WebSocket, channels: WsChannels) {
    // Add this client to the broadcast list
    {
        let mut clients = channels.clients.lock().unwrap();
        clients.push(socket.clone());
    }

    // Main receive loop
    while let Some(Ok(msg)) = socket.recv().await {
        match msg {
            Message::Text(text) => {
                // Echo back to sender
                let _ = socket.send(Message::Text(format!("echo: {}", text))).await;

                // Broadcast to all clients
                channels.broadcast(&text).await;
            }
            Message::Binary(bin) => {
                let _ = socket.send(Message::Binary(bin)).await;
            }
            Message::Close(_) => break,
            _ => {}
        }
    }

    // Remove client on disconnect
    {
        let mut clients = channels.clients.lock().unwrap();
        clients.retain(|c| !std::ptr::eq(c, &socket));
    }
}
