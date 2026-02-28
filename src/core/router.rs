use crate::AppState;
use crate::core::controllers::ws_controller::ws_handler;
use axum::{Router, routing::get};
use std::sync::Arc;

/// Initializes the application's application routing logic.
pub fn initialize_router(state: Arc<AppState>) -> Router {
    Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state)
}
