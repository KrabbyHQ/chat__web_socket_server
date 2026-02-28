use crate::AppState;
use axum::{
    extract::{
        State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::IntoResponse,
};
use std::sync::Arc;
use tracing::{info, warn};

/// Handler for the incoming WebSocket connection upgrade.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

/// Core logic for handling individual WebSocket connections.
async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    info!(
        "New WebSocket connection established from app: {}",
        state.config.app.name
    );

    // Initial greeting
    if socket
        .send(Message::Text(
            "Welcome to the chat WebSocket server!".into(),
        ))
        .await
        .is_err()
    {
        warn!("Failed to send initial greeting");
        return;
    }

    while let Some(msg) = socket.recv().await {
        match msg {
            Ok(Message::Text(text)) => {
                info!("Received text: {}", text);
                // Simple echo for now
                if socket
                    .send(Message::Text(format!("Echo: {}", text).into()))
                    .await
                    .is_err()
                {
                    warn!("Failed to send echo response");
                    break;
                }
            }
            Ok(Message::Binary(bin)) => {
                info!("Received binary data layer of size: {}", bin.len());
            }
            Ok(Message::Close(c)) => {
                if let Some(cf) = c {
                    info!("WebSocket closed: code={}, reason={}", cf.code, cf.reason);
                } else {
                    info!("WebSocket closed without frame");
                }
                break;
            }
            Err(e) => {
                warn!("WebSocket error: {}", e);
                break;
            }
            _ => {}
        }
    }

    info!("WebSocket connection terminated");
}
